mod podmanager;

use std::env;
use std::sync::Mutex;
use std::cmp::min;
use std::fmt::Write;

use lazy_static::lazy_static;
use podmanager::{PodManager, ExecResult};
use regex::{Regex, RegexBuilder};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::UserId;
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

fn zws_encode(text: String) -> String {
    text.replace("`", "`\u{200B}")
}

fn format_output(output: ExecResult) -> String {
    const MAXLEN: isize = 1800; // 200 extra chars for trailing stuff
    const ERRMAXLEN: isize = 1500; // Some space for non-error output

    let mut add_ws = false;
    let mut formatted = "".to_string();
    if !output.status.success() {
        add_ws = true;
        if let Some(code) = output.status.code() {
            formatted += "Error: Exited with exit code";
            let _ = write!(formatted, " {}", code);
        } else {
            formatted += "Error: Exited due to signal";
        }
    }

    if let Some(stderr) = output.stderr {
        if add_ws {
            formatted += "\n\n";
        }

        let encoded = zws_encode(stderr);
        formatted += "STDERR:\n```ansi\n";

        let space = min(MAXLEN - formatted.len() as isize, ERRMAXLEN);
        if space < 0 {
            formatted += "(truncated...)";
        } else if encoded.len() > space as usize {
            formatted += &encoded[0..space as usize];
            formatted += " (truncated...)";
        } else {
            formatted += &encoded;
        }

        formatted += "\n```\n";
    }

    if let Some(stdout) = output.stdout {
        if add_ws {
            formatted += "\n\n";
        }

        let encoded = zws_encode(stdout);
        formatted += "Result:\n```\n";

        let space = MAXLEN - formatted.len() as isize;
        if space < 0 {
            formatted += "(truncated...)";
        } else if encoded.len() > space as usize {
            formatted += &encoded[0..space as usize];
            formatted += " (truncated...)";
        } else {
            formatted += &encoded;
        }

        formatted += "\n```";
    }

    formatted
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

        if let Err(err) = msg.reply(&ctx.http, format_output(output)).await {
            eprintln!("Error sending response: {}", err);
        }
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
