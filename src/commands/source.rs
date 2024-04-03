use serenity::builder::CreateCommand;

pub fn register() -> CreateCommand {
    CreateCommand::new("source").description("Send github link")
}