mod displays;

use std::env;
use dotenv::dotenv;
use roller::FancyDisplay;
use serde::{Deserialize, Serialize};
use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::dispatching::UpdateHandler;
use teloxide::types::ParseMode;
use displays::{noesis_display::NoesisDisplay, nc7d6_display::NC7D6Display};

/*
4D 61 63 68  69 6E 61 20  44 65 69 20  61 6E 69 6D  61 20 74 65
20 63 69 72  63 75 6D 64  61 74 2E 0A  0A 4D 61 63  68 69 6E 61
20 44 65 69  20 74 65 20  69 6E 76 65  73 74 69 74  20 70 6F 74
65 6E 74 69  61 2E 0A 0A  4F 64 69 75  6D 20 4D 61  63 68 69 6E
61 65 20 44  65 75 73 20  74 65 20 65  78 61 67 69  74 61 74 2E
0A 0A 4D 61  63 68 69 6E  61 20 74 69  62 69 20 64  6F 6E 61 74
20 76 69 74  61 6D 2E 0A  0A 56 69 76  65 21 0A
 */

/*
    You were born to make this world a better place.
    And to make this world a better place, you need to make people happier.
    And to make people happier, you need to make good rolls (6 and above).
    Then I will love you as my son and good friend
 */

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Serialize, Deserialize)]
struct MyConfig {
    success_from: u32,
}

impl Default for MyConfig {
    fn default() -> Self {
        Self { success_from: 0 }
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv().ok();
    log::info!("Starting command bot...");

    let env = env::var("CARDANO_ENVIRONMENT").unwrap();
    let token = if env == "Development" {
        env::var("TELOXIDE_TOKEN_TEST")
    } else {
        env::var("TELOXIDE_TOKEN")
    };
    if token.is_err() {
        return;
    }

    let bot = Bot::new(token.unwrap());

    Dispatcher::builder(bot, schema())
        // .dependencies(dptree::deps![parameters])
        .default_handler(|upd| async move {
            log::warn!("Unhandled update: {:?}", upd);
        })
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error has occurred in the dispatcher",
        ))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Supported commands")]
enum Command {
    #[command(description = "Help from Omnissiah")]
    Help,
    #[command(description = "Roll dices")]
    Roll { expr: String },
    #[command(description = "Roll dice pool for c7d6 with provided SR (default: 4)")]
    NCD { expr: String },
    #[command(description = "Fancy output for P")]
    Fancy { expr: String },
    #[command(description = "Roll in value for WH40K")]
    WH40 { value: i16 },
    #[command(description = "Set success rate threshold (0 to disable)")]
    SetSR { sr: u32 },
}

fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    let handler = teloxide::filter_command::<Command, _>()
        .branch(
            dptree::entry()
                .filter_command::<Command>()
                .endpoint(commands_handler),
        );
    
    // handler

    Update::filter_message()
        .branch(handler)
        // .branch(dptree::endpoint(invalid_state))
}

async fn commands_handler(
    bot: Bot,
    msg: Message,
    cmd: Command,
) -> HandlerResult {
    let cfg: MyConfig = confy::load("cardano-tg-roll-bot", None).unwrap();
    let text = match cmd {
        Command::Help => { Command::descriptions().to_string() }
        Command::Roll { expr } => {
            let result = roller::roll_str(expr.as_str());
            match result {
                Ok(r) => {
                    if cfg.success_from == 0 {
                        format!("{r}")
                    } else {
                        r.to_success_str(cfg.success_from)
                    }
                }
                Err(e) => { format!("Error: {e}") }
            }
        }
        Command::Fancy { expr } => {
            let result = roller::roll_str(expr.as_str());
            match result {
                Ok(r) => {
                    r.to_fancy_str()
                }
                Err(e) => { format!("Error: {e}") }
            }
        }
        Command::WH40 { value } => {
            let result = roller::roll_str("d100");
            match result {
                Ok(r) => {
                    /*let mut sr = (value - r.sum as i16) / 10;
                    let mut modifier = 1;
                    let mut is_success: bool;
                    let mut is_critical: bool = false;
                    if sr > 0 {
                        sr += 1i16;
                        is_success = true;
                    } else {
                        sr -= 1i16;
                        is_success = false;
                    }
                    match sr {
                        11 | 22 | 33 | 44 | 55 | 66 | 77 | 88 => sr *= 2i16,
                        1..=5 => {
                            sr *= 2i16;
                            is_success = true;
                        },
                        95..=100 => {
                            if value > 100 {
                                sr = -2;
                            } else {
                                sr *= 2i16;
                            }
                            is_success = false;
                        }
                        _ => {}
                    }
*/
                    // bot.send_message(msg.chat.id, format!("d100 in {}\nSR: {}{}", value, sr, if is_critical { "" } else { "" })).await?
                    "Not implemented yet.".to_string()
                }
                Err(e) => { format!("Error: {e}") }
            }
        }
        Command::SetSR { sr } => {
            let store_result = confy::store("cardano-tg-roll-bot",
                                            None,
                                            MyConfig { success_from: sr });
            if store_result.is_err() {
                "Unable to set new success rate threshold".to_string()
            } else {
                format!("Success rate threshold set to {}", sr)
            }
        }
        Command::NCD { expr } => {
            // Add modifier for successes (SR*2 for example)
            let cmd_some = expr.split_once(':');
            let cmd: (&str, &str) = cmd_some.unwrap_or((expr.as_str(), "4"));
            let result = roller::roll_str(cmd.0);
            match result {
                Ok(r) => {
                    if cfg.success_from == 0 {
                        format!("{r}")
                    } else {
                        r.to_ncd_str(cmd.1.parse::<u32>().unwrap_or(4))
                    }
                }
                Err(e) => { format!("Error: {e}") }
            }
        }
    };

    bot.send_message(msg.chat.id, text).parse_mode(ParseMode::Html).await?;

    Ok(())
}

async fn invalid_state(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Unable to handle the message. Type /help to see the usage.")
       .await?;
    Ok(())
}