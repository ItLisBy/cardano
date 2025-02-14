use roller::RollResult;

pub trait NC7D6Display {
    fn to_ncd_str(&self, sr: u32) -> String;
}

impl NC7D6Display for RollResult {
    fn to_ncd_str(&self, sr: u32) -> String {
        let mut s = String::new();
        let mut success_rate: i16 = 0;
        let mut flag: u8 = 0; // 0b0000|0000 - first half for double 1s; second for double maxes
        for i in self.value.iter() {
            if sr != self.dice as u32 && *i == self.dice as u32 {
                s.push_str(format!("<u>d{} -> {}</u>\n", self.dice, i).as_str());
                success_rate += 1;
                if (flag & 0b00001111) == 0b00001111 {
                    flag ^= 0b00001111;
                    success_rate += 2;
                } else {
                    flag ^= 0b00001111;
                }
            } else if *i == 1 {
                s.push_str(format!("<u>d{} -> {}</u>\n", self.dice, i).as_str());
                if (flag & 0b11110000) == 0b11110000 {
                    flag ^= 0b11110000;
                    success_rate -= 2;
                } else {
                    flag ^= 0b11110000;
                }
            } else if *i >= sr {
                s.push_str(format!("<b>d{} -> {}</b>\n", self.dice, i).as_str());
                success_rate += 1;
            } else {
                s.push_str(format!("d{} -> {}\n", self.dice, i).as_str());
            }
        }
        s.push_str(format!("{}", self.sum).as_str());
        if self.number > 4u16 {
            s.push_str(format!("\n____\nSuccesses: {}", success_rate).as_str());
        }
        s
    }
}