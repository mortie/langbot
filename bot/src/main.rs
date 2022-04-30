mod podmanager;

use std::env;
use std::sync::Mutex;

use lazy_static::lazy_static;
use podmanager::PodManager;
use regex::{Regex, RegexBuilder};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::UserId;
use serenity::utils::Color;
use serenity::prelude::*;

lazy_static! {
    static ref MULTILINE_CODE_RX: Regex = {
        let pattern = r"^<@\d+>\s+(\w+).*```\w*(.*?)```";
        RegexBuilder::new(pattern)
            .dot_matches_new_line(true)
            .build()
            .unwrap()
    };
    static ref INLINE_CODE_RX: Regex = {
        let pattern = r"^<@\d+>\s+(\w+).*`(.*?)`";
        Regex::new(pattern).unwrap()
    };
}

struct Handler {
    id: Mutex<Option<UserId>>,
    podman: Mutex<podmanager::PodManager>,
}

fn exit_code_to_desc(code: i32) -> Option<&'static str> {
    match code {
        126 => Some("Command not executable"),
        127 => Some("Command  not found"),
        129 => Some("SIGHUP"),
        130 => Some("SIGINT"),
        131 => Some("SIGQUIT"),
        132 => Some("SIGILL"),
        133 => Some("SIGTRAP"),
        134 => Some("SIGABRT"),
        135 => Some("SIGBUS"),
        136 => Some("SIGFPE"),
        137 => Some("SIGKILL"),
        139 => Some("SIGSEGV"),
        141 => Some("SIGPIPE"),
        143 => Some("SIGTERM"),
        _ => None,
    }
}

fn zws_encode(text: String) -> String {
    text.replace("`", "`\u{200B}")
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Ignore messages from bots (avoid endless loop)
        if msg.author.bot {
            return;
        }

        // Ignore messages which don't mention us
        let my_id = self.id.lock().unwrap().unwrap();
        if msg.mentions.iter().find(|&m| m.id == my_id).is_none() {
            return;
        }

        let caps = MULTILINE_CODE_RX
            .captures(&msg.content)
            .or_else(|| INLINE_CODE_RX.captures(&msg.content));
        let caps = match caps {
            Some(caps) => caps,
            None => return,
        };

        let language =  caps.get(1).unwrap().as_str().to_lowercase();
        let content = caps.get(2).unwrap().as_str();

        let _typing = msg.channel_id.start_typing(&ctx.http);

        let pod = self.podman.lock().unwrap().get_pod();
        let pod = match pod {
            Ok(pod) => pod,
            Err(err) => {
                eprintln!("Couldn't get pod: {}", err);
                if let Err(err) = msg.channel_id.say(&ctx.http, format!("Error: {}", err)).await {
                    eprintln!("Error sending response: {}", err);
                }
                return;
            }
        };

        let output = match pod.execute(&language, &content) {
            Ok(output) => output,
            Err(err) => {
                eprintln!("Couldn't execute code: {}", err);
                if let Err(err) = msg.channel_id.say(&ctx.http, format!("Error: {}", err)).await {
                    eprintln!("Error sending response: {}", err);
                }
                return;
            }
        };

        let res = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                if !output.status.success() {
                    let code = match output.status.code() {
                        Some(code) => match exit_code_to_desc(code) {
                            Some(desc) => format!("{} ({})", code, desc),
                            None => format!("{}", code),
                        },
                        None => "Signal".to_string(),
                    };

                    e.description(format!("Exit Code {}", code));
                    e.color(Color::DARK_RED);
                } else {
                    e.color(Color::DARK_GREEN);
                }

                if let Some(stdout) = output.stdout {
                    e.field("STDOUT", format!("```ansi\n{}\n```", zws_encode(stdout)), false);
                }

                if let Some(stderr) = output.stderr {
                    e.field("STDERR", format!("```ansi\n{}\n```", zws_encode(stderr)), false);
                }

                e
            })
        }).await;
        if let Err(err) = res {
            eprintln!("Error sending response: {}", err);
            let _ = msg.channel_id.say(&ctx.http, format!("Couldn't send response: {}", err)).await;
        }

        /*
        if let Err(err) = msg.reply(&ctx.http, format_output(output)).await {
            eprintln!("Error sending response: {}", err);
        }
        */
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        *self.id.lock().unwrap() = Some(ready.user.id);
        eprintln!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a DISCORD_TOKEN in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let handler = Handler {
        id: Mutex::new(None),
        podman: Mutex::new(PodManager::new("langbot".into())),
    };

    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}
