use roller::RollResult;

pub trait FancyDisplay {
    fn to_fancy_str(&self) -> String;
}

impl FancyDisplay for RollResult {
    fn to_fancy_str(&self) -> String {
        let mut s = String::new();
        s.push_str(format!("{}d{}\n", self.number, self.dice).as_str());
        for i in self.value.iter() {
            s.push_str(format!("-> {}\n", i).as_str());
        }
        s.push_str(format!("____\n{}", self.sum).as_str());
        s
    }
}