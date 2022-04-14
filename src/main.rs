use crossterm::terminal;
use std::io::{self, Read};

fn main() {
    let _enable_raw_mode_result = terminal::enable_raw_mode();

    for b in io::stdin().bytes() {
        let b = b.unwrap();
        let c = b as char;
        if c.is_control() {
            println!("{:#b} \r", b);
        } else {
            println!("{:#b} ({})\r", b, c);
        }
        if b == 0b11 {
            break;
        }
    }
}
