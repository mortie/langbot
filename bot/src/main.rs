mod podmanager;

use std::collections::HashMap;
use std::env;
use std::sync::Mutex;

use lazy_static::lazy_static;
use podmanager::{ExecResult, PodManager};
use regex::{Regex, RegexBuilder};
use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::model::channel::Message;
use serenity::model::event::MessageUpdateEvent;
use serenity::model::gateway::Ready;
use serenity::model::id::{ChannelId, MessageId, UserId};
use serenity::prelude::*;
use serenity::utils::Color;

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

fn truncate_string(text: &String) -> String {
    const CHLIMIT: usize = 800;
    const LINELIMIT: usize = 20;

    let mut numlines = 0;
    let mut output = "".to_string();
    let mut truncated = false;
    for line in text.lines() {
        if output.len() > 0 {
            output += "\n";
        }

        output += line;
        numlines += 1;
        if numlines >= LINELIMIT || output.len() >= CHLIMIT {
            truncated = true;
            break;
        }
    }

    if output.len() > CHLIMIT {
        format!(
            "{} (truncated...)",
            output.chars().take(CHLIMIT).collect::<String>()
        )
    } else if truncated {
        format!("{} (truncated...)", output)
    } else {
        output
    }
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

struct Handler {
    id: Mutex<Option<UserId>>,
    podman: Mutex<podmanager::PodManager>,
    responses: Mutex<HashMap<(ChannelId, MessageId), (ChannelId, MessageId)>>,
}

fn create_embed_from_result(output: ExecResult, embed: &mut CreateEmbed) {
    if !output.status.success() {
        let code = match output.status.code() {
            Some(code) => match exit_code_to_desc(code) {
                Some(desc) => format!("{} ({})", code, desc),
                None => format!("{}", code),
            },
            None => "Signal".to_string(),
        };

        embed.description(format!("Exit Code {}", code));
        embed.color(Color::DARK_RED);
    } else {
        embed.description(format!("Exit Code 0 (OK)"));
        embed.color(Color::DARK_GREEN);
    }

    if let Some(stdout) = &output.stdout {
        embed.field(
            "STDOUT",
            format!("```ansi\n{}\n```", zws_encode(truncate_string(stdout))),
            false,
        );
    }

    if let Some(stderr) = &output.stderr {
        embed.field(
            "STDERR",
            format!("```ansi\n{}\n```", zws_encode(truncate_string(stderr))),
            false,
        );
    }
}

impl Handler {
    fn parse_and_run_text(&self, text: &str) -> Option<Result<ExecResult, String>> {
        let caps = MULTILINE_CODE_RX
            .captures(text)
            .or_else(|| INLINE_CODE_RX.captures(text));
        let caps = match caps {
            Some(caps) => caps,
            None => return None,
        };

        let language = caps.get(1).unwrap().as_str().to_lowercase();
        let content = caps.get(2).unwrap().as_str();

        let pod = self.podman.lock().unwrap().get_pod();
        let pod = match pod {
            Ok(pod) => pod,
            Err(err) => return Some(Err(err)),
        };

        let output = match pod.execute(&language, &content) {
            Ok(output) => output,
            Err(err) => return Some(Err(err)),
        };

        Some(Ok(output))
    }

    fn parse_and_run(&self, msg: &Message) -> Option<Result<ExecResult, String>> {
        // Ignore messages from bots
        if msg.author.bot {
            return None;
        }

        // Ignore messages which don't mention us
        let my_id = self.id.lock().unwrap().unwrap();
        if msg.mentions.iter().find(|&m| m.id == my_id).is_none() {
            return None;
        }

        self.parse_and_run_text(&msg.content)
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message_update(&self, ctx: Context, evt: MessageUpdateEvent) {
        let response = match self
            .responses
            .lock()
            .unwrap()
            .get(&(evt.channel_id, evt.id))
        {
            Some(response) => *response,
            None => return,
        };
        let (response_channel, response_id) = response;

        let content = match evt.content {
            Some(content) => content,
            None => return,
        };

        let output = match self.parse_and_run_text(&content) {
            Some(output) => output,
            None => return,
        };

        let output = match output {
            Ok(output) => output,
            Err(err) => {
                eprintln!("Error: {}", err);
                let resp = response_channel
                    .edit_message(&ctx.http, response_id, |edit| {
                        edit.content(format!("Error: {}", err))
                    })
                    .await;
                if let Err(err) = resp {
                    eprintln!("Couldn't edit message: {}", err);
                }
                return;
            }
        };

        let resp = response_channel
            .edit_message(&ctx.http, response_id, |edit| {
                edit.embed(|embed| {
                    create_embed_from_result(output, embed);
                    embed
                })
            })
            .await;
        match resp {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Couldn't edit message: {}", err);
                let resp = response_channel
                    .edit_message(&ctx.http, response_id, |edit| {
                        edit.content(format!("Error: {}", err))
                    })
                    .await;
                if let Err(err) = resp {
                    eprintln!("Couldn't edit message: {}", err);
                }
            }
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let output = match self.parse_and_run(&msg) {
            Some(output) => output,
            None => return,
        };

        let output = match output {
            Ok(output) => output,
            Err(err) => {
                eprintln!("Error: {}", err);
                if let Err(err) = msg
                    .channel_id
                    .say(&ctx.http, format!("Error: {}", err))
                    .await
                {
                    eprintln!("Couldn't send error: {}", err);
                }
                return;
            }
        };

        let resp = msg
            .channel_id
            .send_message(&ctx.http, |m| {
                m.embed(|embed| {
                    create_embed_from_result(output, embed);
                    embed
                })
            })
            .await;
        match resp {
            Ok(reply) => {
                self.responses
                    .lock()
                    .unwrap()
                    .insert((msg.channel_id, msg.id), (reply.channel_id, reply.id));
            }
            Err(err) => {
                eprintln!("Couldn't send message: {}", err);
                if let Err(err) = msg
                    .channel_id
                    .say(&ctx.http, format!("Error: {}", err))
                    .await
                {
                    eprintln!("Couldn't send error: {}", err);
                }
            }
        }
    }

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
        responses: Mutex::new(HashMap::new()),
    };

    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}
