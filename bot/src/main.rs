mod podmanager;

use std::env;
use std::fs;
use std::sync::{Arc, Mutex};
use std::io::Read;
use std::borrow::Cow;

use lazy_static::lazy_static;
use lru::LruCache;
use podmanager::{ExecResult, PodManager};
use regex::{Regex, RegexBuilder};
use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::model::channel::{Message, MessageReference, AttachmentType};
use serenity::model::event::MessageUpdateEvent;
use serenity::model::gateway::Ready;
use serenity::model::id::{ChannelId, MessageId};
use serenity::model::user::CurrentUser;
use serenity::prelude::*;
use serenity::utils::Color;

lazy_static! {
    static ref MULTILINE_CODE_RX: Regex = {
        let pattern = r"!([a-zA-Z][a-zA-Z0-9+_]*)\s+```\S*\s*(.*?)```";
        RegexBuilder::new(pattern)
            .dot_matches_new_line(true)
            .build()
            .unwrap()
    };
    static ref INLINE_CODE_RX: Regex = {
        let pattern = r"!([a-zA-Z][a-zA-Z0-9+_]*)\s+`(.*?)`";
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
    user: Mutex<Option<CurrentUser>>,
    podman: Arc<podmanager::PodManager>,
    responses: Mutex<LruCache<(ChannelId, MessageId), (ChannelId, MessageId)>>,
}

fn create_embed_from_result(output: &ExecResult, embed: &mut CreateEmbed) {
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

fn is_output_interesting(output: &ExecResult) -> bool {
    return !output.status.success() || output.stdout.is_some() || output.stderr.is_some();
}

fn create_attachments(output: &ExecResult) -> Vec<AttachmentType> {
    const MAX_SIZE: u64 = 500 * 1024; // 500kiB
    let mut files = match &output.files {
        Some(files) => files.lock().unwrap(),
        None => return Vec::new(),
    };

    let entries = match files.entries() {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("Read files error: {}", err);
            return Vec::new();
        }
    };

    let mut attachments: Vec<AttachmentType> = Vec::new();

    let mut total_size = 0;
    for ent in entries {
        let mut ent = match ent {
            Ok(ent) => ent,
            Err(err) => {
                eprintln!("Read files error: {}", err);
                return attachments;
            }
        };

        let entsize = ent.size();
        if entsize == 0 {
            continue;
        }

        total_size += entsize;
        if total_size > MAX_SIZE {
            eprintln!("Files too large!");
            return attachments;
        }

        let path = match ent.path() {
            Ok(path) => path.into_owned().to_string_lossy().to_string(),
            Err(err) => {
                eprintln!("Invalid file name: {}", err);
                return attachments;
            }
        };

        let mut buf: Vec<u8> = Vec::new();
        buf.resize(entsize as usize, 0u8);
        let size = match ent.read(buf.as_mut_slice()) {
            Ok(size) => size,
            Err(err) => {
                eprintln!("Invalid read: {}", err);
                return attachments;
            }
        };
        buf.truncate(size); // In case we read less than what it said

        attachments.push(AttachmentType::Bytes{
            data: Cow::Owned(buf),
            filename: path,
        });
    }

    attachments
}

impl Handler {
    fn parse_and_run(&self, text: &str) -> Option<Result<ExecResult, String>> {
        let caps = MULTILINE_CODE_RX
            .captures(text)
            .or_else(|| INLINE_CODE_RX.captures(text));
        let caps = match caps {
            Some(caps) => caps,
            None => return None,
        };

        let language = caps.get(1).unwrap().as_str().to_lowercase();
        let content = caps.get(2).unwrap().as_str();

        let pod = self.podman.get_pod();
        let mut pod = match pod {
            Ok(pod) => pod,
            Err(err) => return Some(Err(err)),
        };

        let output = match pod.execute(&language, &content) {
            Ok(output) => output,
            Err(err) => return Some(Err(err)),
        };

        Some(Ok(output))
    }

    fn does_message_mention_us(&self, msg: &Message) -> bool {
        // If the message is a response, we don't wanna care
        if let Some(_) = msg.referenced_message {
            return false;
        }

        let me = self.user.lock().unwrap();
        let my_id = me.as_ref().unwrap().id;
        if msg.mentions.iter().find(|&m| m.id == my_id).is_some() {
            return true;
        }

        false
    }

    async fn send_usage_info(&self, ctx: Context, msg: Message) {
        let message = {
            let me = self.user.lock().unwrap();
            let name = &me.as_ref().unwrap().name;
            let mut msg = format!(
                "I'm {}! I can be used to run code in all kinds of languages. Try this:
```
!language `source code`
```
Or this:
```
!language
`\u{200B}``
source code
`\u{200B}``
```",
                name
            );

            if let Ok(paths) = fs::read_dir("../langs") {
                let mut names = Vec::new();
                for path in paths {
                    if let Ok(path) = path {
                        names.push(path.file_name().to_string_lossy().to_string());
                    }
                }

                names.sort();
                msg += "\nI support these languages: ";
                let mut first = true;
                for name in names {
                    if !first {
                        msg += ", ";
                    } else {
                        first = false;
                    }

                    msg += "`";
                    msg += &name;
                    msg += "`";
                }
            }

            msg += "\nFor more info, check out: <https://github.com/mortie/langbot>";

            msg
        };

        if let Err(err) = msg.channel_id.say(&ctx.http, message).await {
            eprintln!("Couldn't send message: {}", err);
        }
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

        let output = match self.parse_and_run(&content) {
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
                let attachments = create_attachments(&output);
                if attachments.is_empty() || is_output_interesting(&output) {
                    edit.embed(|embed| {
                        create_embed_from_result(&output, embed);
                        embed
                    });
                }
                for attachment in attachments {
                    edit.attachment(attachment);
                }
                edit
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
        // Ignore messages from bots
        if msg.author.bot {
            return;
        }

        let output = match self.parse_and_run(&msg.content) {
            Some(output) => output,
            None => {
                if self.does_message_mention_us(&msg) {
                    self.send_usage_info(ctx, msg).await;
                }
                return;
            }
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
                m.reference_message(MessageReference::from((msg.channel_id, msg.id)));
                m.allowed_mentions(|a| {
                    a.empty_parse()
                });
                let attachments = create_attachments(&output);
                if attachments.is_empty() || is_output_interesting(&output) {
                    m.embed(|embed| {
                        create_embed_from_result(&output, embed);
                        embed
                    });
                }
                for attachment in attachments {
                    m.add_file(attachment);
                }
                m
            })
            .await;
        match resp {
            Ok(reply) => {
                self.responses
                    .lock()
                    .unwrap()
                    .put((msg.channel_id, msg.id), (reply.channel_id, reply.id));
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
        eprintln!("{} is connected!", ready.user.name);
        *self.user.lock().unwrap() = Some(ready.user);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a DISCORD_TOKEN in the environment");
    env::remove_var("DISCORD_TOKEN"); // Don't accidentally pass the token to child processes
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let handler = Handler {
        user: Mutex::new(None),
        podman: Arc::new(PodManager::new("langbot".into())),
        responses: Mutex::new(LruCache::new(1024)),
    };

    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}
