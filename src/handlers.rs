use std::fmt::format;
use std::ops::Neg;
use regex::Regex;
use crate::displays::fancy_display::FancyDisplay;
use crate::displays::nc7d6_display::NC7D6Display;
use crate::displays::noesis_display::NoesisDisplay;
use crate::MyConfig;

pub fn roll_handler(expr: String, cfg: MyConfig) -> String {
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

pub fn fancy_handler(expr: String) -> String {
    let result = roller::roll_str(expr.as_str());
    match result {
        Ok(r) => {
            r.to_fancy_str()
        }
        Err(e) => { format!("Error: {e}") }
    }
}

pub fn wh40k_handler(value: i16) -> String {
    let result = roller::roll_str("d100");
    match result {
        Ok(r) => {
            let mut sr = (value - r.sum as i16) / 10;
            let mut is_success: bool = true;
            let mut is_critical: bool = false;
            match sr {
                x if x > 0 => {
                    sr += 1i16;
                    is_success = true;
                }
                0 => {
                    if value < r.sum as i16 {
                        sr = -1;
                        is_success = false;
                    } else {
                        sr = 1;
                        is_success = true;
                    }
                }
                x if x < 0 => {
                    // sr -= 1i16;
                    is_success = false;
                }
                _ => {}
            }
            match r.sum {
                11 | 22 | 33 | 44 | 55 | 66 | 77 | 88 => {
                    sr *= 2i16;
                    is_critical = true;
                }
                1..=5 => {
                    sr *= 2i16;
                    is_success = true;
                    is_critical = true;
                }
                95..=100 => {
                    if value > 100 {
                        sr = -2;
                    } else {
                        sr *= 2i16;
                    }
                    is_success = false;
                    is_critical = true;
                }
                _ => {}
            }

            sr = sr.checked_abs().unwrap();
            if !is_success {
                sr = sr.checked_neg().unwrap();
            }

            if is_critical {
                format!("d100: {} in {}\n<u>SR: {}</u>", r.sum, value, sr).to_string()
            } else {
                format!("d100: {} in {}\nSR: {}", r.sum, value, sr).to_string()
            }
        }
        Err(e) => { format!("Error: {e}") }
    }
}

pub fn set_sr_handler(sr: u32) -> String {
    let store_result = confy::store("cardano-tg-roll-bot",
                                    None,
                                    MyConfig { success_from: sr });
    if store_result.is_err() {
        "Unable to set new success rate threshold".to_string()
    } else {
        format!("Success rate threshold set to {}", sr)
    }
}

pub fn ncd_handler(expr: String) -> String {
    let re1 = Regex::new(r"(?<num>\d*)d(?<dice>\d+)(:(?<sr>\d+))?(?<mods>.*)").unwrap();
    let cmd = re1.captures(expr.as_str()).unwrap();
    let clear_expr = format!("{}d{}{}", cmd.name("num").unwrap().as_str(), cmd.name("dice").unwrap().as_str(), cmd.name("mods").unwrap().as_str());
    let result = roller::roll_str(clear_expr.as_str());
    match result {
        Ok(r) => {
            let sr = match cmd.name("sr") {
                Some(x) => x.as_str().parse::<u32>().unwrap(),
                None => (cmd.name("dice").unwrap().as_str().parse::<u32>().unwrap() / 2) + 1u32,
            };
            r.to_ncd_str(sr)
        }
        Err(e) => { format!("Error: {e}") }
    }
}