use std::fmt::format;
use std::ops::Neg;
use regex::{Match, Regex};
use crate::displays::fancy_display::FancyDisplay;
use crate::displays::nc7d6_display::NC7D6Display;
use crate::displays::noesis_display::NoesisDisplay;
use crate::MyConfig;

pub fn roll_handler(expr: String, cfg: MyConfig) -> String {
    let result = roller::roll_str(expr.as_str());
    match result {
        Ok(r) => {
            format!("{r}")
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

pub fn dh_handler(expr: String) -> String {
    let result = roller::roll_str("d100");
    match result {
        Ok(r) => {
            let re1 = Regex::new(r"(?<value>\d+)(?<adv>[aAdD]?)$").unwrap();
            let value_parsed = re1.captures(&expr).unwrap();

            let mut result_value = r.sum as i16;
            match value_parsed.name("adv") {
                None => {}
                Some(adv) => {
                    let swapped = swap_digits(result_value);
                    if ((adv.as_str() == "d" || adv.as_str() == "D") && swapped > result_value)
                        || ((adv.as_str() == "a" || adv.as_str() == "A") && swapped < result_value) {
                        result_value = swapped;
                    }
                }
            }

            let value: i16 = match value_parsed.name("value") {
                None => {
                    1i16
                }
                Some(num) => {
                    num.as_str().parse::<i16>().unwrap()
                }
            };
            let mut sr = (value - result_value) / 10;
            let mut is_success: bool = true;
            let mut is_critical: bool = false;
            match sr {
                x if x > 0 => {
                    sr += 1;
                    is_success = true;
                }
                0 => {
                    if value < result_value {
                        sr = -1;
                        is_success = false;
                    } else {
                        sr = 1;
                        is_success = true;
                    }
                }
                x if x < 0 => {
                    is_success = false;
                }
                _ => {}
            }
            match result_value {
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
                format!("d100: {} in {}\n<u>SR: {}</u>", result_value, expr, sr).to_string()
            } else {
                format!("d100: {} in {}\nSR: {}", result_value, expr, sr).to_string()
            }
        }
        Err(e) => { format!("Error: {e}") }
    }
}

pub fn im_handler(expr: String) -> String {
    let result = roller::roll_str("d100");
    match result {
        Ok(r) => {
            let re1 = Regex::new(r"(?<value>\d+)(?<adv>[aAdD]?)$").unwrap();
            let value_parsed = re1.captures(&expr).unwrap();

            let mut result_value = r.sum as i16;
            match value_parsed.name("adv") {
                None => {}
                Some(adv) => {
                    let swapped = swap_digits(result_value);
                    if ((adv.as_str() == "d" || adv.as_str() == "D") && swapped > result_value)
                        || ((adv.as_str() == "a" || adv.as_str() == "A") && swapped < result_value) {
                        result_value = swapped;
                    }
                }
            }

            let value: i16 = match value_parsed.name("value") {
                None => {
                    1i16
                }
                Some(num) => {
                    num.as_str().parse::<i16>().unwrap()
                }
            };
            let mut sr = (value - result_value) / 10;
            let mut is_success: bool = true;
            let mut is_critical: bool = false;
            match sr {
                x if x > 0 => {
                    is_success = true;
                }
                0 => {
                    if value < result_value {
                        sr = -1;
                        is_success = false;
                    } else {
                        sr = 1;
                        is_success = true;
                    }
                }
                x if x < 0 => {
                    is_success = false;
                }
                _ => {}
            }
            match result_value {
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
                format!("d100: {} in {}\n<u>SR: {}</u>", result_value, expr, sr).to_string()
            } else {
                format!("d100: {} in {}\nSR: {}", result_value, expr, sr).to_string()
            }
        }
        Err(e) => { format!("Error: {e}") }
    }
}

fn set_sr_handler(sr: u32) -> String {
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

fn swap_digits(n: i16) -> i16 {
    if n == 100 {
        return 1;
    }
    let tens = n / 10;
    let ones = n % 10;
    ones * 10 + tens
}