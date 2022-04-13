use crossterm::terminal;
use std::io::{self, Read};

fn main() {
    let _enable_raw_mode_result = terminal::enable_raw_mode();

    for b in io::stdin().bytes() {
        let c = b.unwrap() as char;
        println!("{}", c);
        if c == 'q' {
            break;
        }
    }
}
