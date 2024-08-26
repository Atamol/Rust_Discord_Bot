use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::application::command::Command;
use serenity::model::prelude::interaction::{Interaction, InteractionResponseType};
use serenity::model::prelude::Ready;
use std::env;
use std::time::Instant;

const TOKEN: &str = "DISCORD_BOT_TOKEN_HERE";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        // スラッシュコマンドをギルドに登録
        let _command = Command::create_global_application_command(&ctx.http, |command| {
            command.name("ping").description("Returns the bot's ping.")
        })
        .await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            if command.data.name.as_str() == "ping" {
                let start_time = Instant::now();

                if let Err(why) = command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::DeferredChannelMessageWithSource)
                    })
                    .await
                {
                    println!("Cannot send deferred response: {}", why);
                    return;
                }

                let end_time = Instant::now();
                let duration = end_time.duration_since(start_time);
                let ping_ms = duration.as_millis();

                if let Err(why) = command
                    .edit_original_interaction_response(&ctx.http, |response| {
                        response.content(format!("Pong! {}ms", ping_ms))
                    })
                    .await
                {
                    println!("Cannot edit interaction response: {}", why);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = TOKEN;

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
