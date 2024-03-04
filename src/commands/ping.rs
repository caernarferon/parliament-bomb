use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> String {
    "Hello from ParliamentBomb! (parliamentbomb rust binary)".to_string()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("A ping command")
}
