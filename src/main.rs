use crossterm::terminal;
use std::io::{self, Read};

fn die(e: std::io::Error) {
    panic!("{}", e);
}

fn main() {
    let _enable_raw_mode_result = terminal::enable_raw_mode();

    for b in io::stdin().bytes() {
        match b {
            Ok(b) => {
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
            Err(e) => die(e),
        }
    }
}
