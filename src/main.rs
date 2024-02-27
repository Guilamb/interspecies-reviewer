use serenity::{
    async_trait,
    model::channel::Message,
    model::{gateway::Ready},
    prelude::*,
    utils::MessageBuilder, builder::EditMessage
};
use url::{Url, Position};
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn message(&self, ctx: Context, mut msg: Message) {
        // Respond to messages that start with "https://"

        if msg.content.contains("https://") && msg.author.tag() != "InterspeciesReviewer#3272" {


            let message = change_link(msg.content.clone());
            if !message.contains("None") {

                let response = MessageBuilder::new()
                .push("User ")
                .push_bold_safe(&msg.author.name)
                .push(" sent ")
                .push(message)
                .build();

            
                if let Err(why) = msg.reply(&ctx.http, &response).await {
                    println!("Error sending message: {why:?}");
                }
                if let Err(reason) = msg.delete_reactions(&ctx.http).await{
                    println!("Error sending message: {reason:?}");

                }

                let rep = EditMessage::new().suppress_embeds(true);

                
                if let Err(reason) = msg.edit(&ctx.http, rep).await{
                    println!("Error with delete -_- : {reason:?}" );
                };
                
            }
            
        }
    }


    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}


fn change_link(url: String) -> String {
    let issue_list_url = Url::parse(&url);
    let mut final_url = "None".to_string();

    match issue_list_url {
        Ok(url) => {
            // Parsing succeeded, print and use the URL components
            println!("website: {}, path: {}", url.host_str().unwrap_or("N/A"), url.path());
            final_url = replace_social_media(url.host_str().unwrap_or("N/A")).to_string() + &url[Position::BeforePath..];
            println!("final url : {}", final_url.as_str())
        }
        Err(e) => {
            // Parsing failed, print the error
            println!("Error parsing URL: {:?}", e);
        }
    }

    final_url
}

fn replace_social_media(website: &str) -> &str {
    // Convert &str to String for modification
    let mut updated_website = website.to_string();

    if updated_website.contains("www") {
        // Modify the String directly
        updated_website = updated_website.replace("www.", "");
        println!("Replaced www, result: {}", updated_website);
    }

    match updated_website.as_str() {
        "twitter.com" => "https://vxtwitter.com",
        "x.com" => "https://fixvx.com",
        "vm.tiktok.com" => "https://vm.vxtiktok.com",
        "instagram.com" => "https://ddinstagram.com",
        _ => "None",
    }
}

#[tokio::main]
async fn main() {
    let token =  env::var("DISCORD_TOKEN").expect("Token not found");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of a client with the bot token and event handler
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Start the bot
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

