mod commands;
use serenity::{
    async_trait,
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
    model::application::{Command,Interaction},
    model::channel::Message,
    model::{gateway::Ready},
    prelude::*,
    utils::MessageBuilder, builder::EditMessage
};
use url::{Url, Position};
use regex::Regex;
use std::{env};

struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn message(&self, ctx: Context, mut msg: Message) {


        let msg_content = msg.content.clone();
 

        // Check if the message contains "http://" or "https://", and ignore messages from the bot to avoid endless loop
        if (msg_content.contains("https://") || msg_content.contains("http://")) && msg.author.tag() != "InterspeciesReviewer#3272" {

            println!("msg reçu : {}", msg_content);

            let seperator = Regex::new(r"(https?://([\^\-_.~!*'();:@&=+$,/?%#A-z0-9]+))").expect("Invalid regex");
            let spoilers_regex = Regex::new(r"\|\| *(https?://([\^\-_.~!*'();:@&=+$,/?%#A-z0-9]+)) *\|\|").expect("Invalid spoiler regex");
            
            

            let  mut raw_url = "None";
            
            // Capture the URL using the regex pattern
            if let Some(captures) = seperator.captures(msg_content.as_str()) {
                // Extract the URL from the first capturing group
                if let Some(url) = captures.get(1) {
                    // Get the matched URL as a string
                    raw_url = url.as_str();
                }
            }

            let mut message = change_link(raw_url.to_string());

            if spoilers_regex.is_match(msg_content.as_str()) {
                message = format!("|| {message} ||");
            }
            


            if !message.contains("None") {


                // Build a response message
                let response = MessageBuilder::new()
                .push("User ")
                .push_bold_safe(&msg.author.name)
                .push(" sent ")
                .push(message)
                .build();

            
                // Send the response message
                if let Err(why) = msg.reply(&ctx.http, &response).await {
                    println!("Error sending message: {why:?}");
                }

                // Delete reactions from the original message
                if let Err(reason) = msg.delete_reactions(&ctx.http).await{
                    println!("Error sending message: {reason:?}");

                }

                // Edit the original message to delete embeds
                let rep = EditMessage::new().suppress_embeds(true);
                if let Err(reason) = msg.edit(&ctx.http, rep).await{
                    println!("Error with delete -_- : {reason:?}" );
                };
                
            }
            
        }


    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction){
        if let Interaction::Command(command) = interaction {
            println!("Interaction received");
            match command.data.name.as_str(){
                "source" => {
                    let data = CreateInteractionResponseMessage::new().content("https://github.com/Guilamb/interspecies-reviewer/issues");
                    let builder = CreateInteractionResponse::Message(data);
                    if let Err(why) = command.create_response(&ctx.http, builder).await {
                        println!("Cannot respond to slash command: {why}");
                    }
                },
                _ => println!("Not implemented"),

            };
        }
    }


    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);


        Command::create_global_command(&ctx.http, commands::source::register())
            .await
            .expect("Failed to create global command");
    }
}



fn change_link(url: String) -> String {
    let issue_list_url = Url::parse(&url);
    let mut final_url = "None".to_string();


    //Attempt to parse the URL, and if succeded print and use the URL components
    match issue_list_url {
        Ok(url) => {
            //println!("website: {}, path: {}", url.host_str().unwrap_or("N/A"), url.path());
            final_url = replace_social_media(url.host_str().unwrap_or("N/A")).to_string() + &url[Position::BeforePath..];
            //println!("final url : {}", final_url.as_str())
        }
        Err(e) => {
            println!("Error parsing URL: {:?}", e);
        }
    }

    final_url
}

fn replace_social_media(website: &str) -> &str {
    // Convert &str to String for modification
    let mut updated_website = website.to_string();

    if updated_website.contains("www") {
        updated_website = updated_website.replace("www.", "");
    }

    // Replace social media URLs with fixed URLs
    match updated_website.as_str() {
        "twitter.com" => "https://vxtwitter.com",
        "x.com" => "https://fixvx.com",
        "vm.tiktok.com" => "https://vm.vxtiktok.com",
        "instagram.com" => "https://ddinstagram.com",
        "pixiv.net" => "https://phixiv.net",
        _ => "None",
    }
}




#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Token not found");
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