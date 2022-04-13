use crossterm::terminal;
use std::io::{self, Read};

fn main() {
    let _enable_raw_mode_result = terminal::enable_raw_mode();

    for b in io::stdin().bytes() {
        let b = b.unwrap();
        let c = b as char;
        if c.is_control() {
            println!("{:?} \r", b);
        } else {
            println!("{:?} ({})\r", b, c);
        }
        if c == 'q' {
            break;
        }
    }
}
