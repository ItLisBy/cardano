use roller::FancyDisplay;
use teloxide::{prelude::*, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    if dotenv::dotenv().is_ok() {
        let token = dotenv::var("TELOXIDE_TOKEN");
        if token.is_ok() {
            let bot = Bot::new(token.unwrap());

            Command::repl(bot, answer).await;
        }
    } else {
        panic!("No .env file found!");
    }
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Supported commands")]
enum Command {
    #[command(description = "Help from Omnissiah")]
    Help,
    #[command(description = "Roll dices")]
    Roll(String),
    #[command(description = "Fancy output for P")]
    Fancy(String),
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Roll(command) => {
            let result = roller::roll_str(command.as_str());
            match result {
                Ok(r) => {
                    bot.send_message(msg.chat.id, format!("{r}")).await?
                }
                Err(e) => { bot.send_message(msg.chat.id, format!("Error: {e}")).await? }
            }
        }
        Command::Fancy(command) => {
            let result = roller::roll_str(command.as_str());
            match result {
                Ok(r) => {
                    bot.send_message(msg.chat.id, r.to_fancy_str()).await?
                }
                Err(e) => { bot.send_message(msg.chat.id, format!("Error: {e}")).await? }
            }
        }
    };

    Ok(())
}
