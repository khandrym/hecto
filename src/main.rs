use crossterm::{
    event::{read, Event, KeyCode, KeyModifiers},
    terminal,
};

fn die(e: std::io::Error) {
    panic!("{}", e);
}

fn main() {
    let _enable_raw_mode_result = terminal::enable_raw_mode();

    loop {
        match read() {
            Ok(event) => match event {
                Event::Key(key) => {
                    if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
                        break;
                    }
                    println!("{:?}", key)
                }
                _ => println!("{:?}", event),
            },
            Err(e) => die(e),
        }
    }
}
