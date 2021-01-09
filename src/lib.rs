use std::io;
use std::io::Write;

pub struct ProgressBar {
    left: String,
    right: String,
    fill: String,
    empty: String,
    width: i32,
    percentage: i32,
    text: String,
}

impl ProgressBar {
    pub fn new() -> ProgressBar {
        ProgressBar {
            left: String::from("["),
            right: String::from("]"),
            fill: String::from("#"),
            empty: String::from(" "),
            width: 30,
            percentage: 0,
            text: String::from(""),
        }
    }
    pub fn show(&self) {
        // 4 is the width of the letters on both ends and the percentage(xxx%)
        let bar_len = self.width - 4;
        let fill_len = (self.percentage as f32 * 0.01 * bar_len as f32) as i32;
        let empty_len = bar_len - fill_len;
        io::stdout().flush().unwrap();
        print!("\r");
        print!("{}", self.left);
        for _ in 0..fill_len {
            print!("{}", self.fill);
        }
        for _ in 0..empty_len {
            print!("{}", self.empty);
        }
        print!("{}", self.right);
        print!("{:^3}%", self.percentage);
        print!("  {}", self.text);
    }
    pub fn increment(&mut self) {
        self.percentage += 1;
    }
    pub fn increment_and_show(&mut self) {
        self.increment();
        self.show();
    }
    pub fn set_percentage(&mut self, percentage: i32) {
        self.percentage = percentage;
    }
    pub fn set_text(&mut self, text: &str) {
        self.text = String::from(text);
    }
}