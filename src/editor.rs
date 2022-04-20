use crossterm::{
    event::{read, Event, KeyCode, KeyModifiers},
    terminal,
};

pub struct Editor {}

impl Editor {
    /// Function to run editor.
    pub fn run() {
        let _enable_raw_mode_result = terminal::enable_raw_mode();

        loop {
            if let Err(error) = process_keypress() {
                die(&error.to_string());
            }
        }
    }
}

fn process_keypress() -> Result<(), std::io::Error> {
    let pressed_key = read()?;
    match pressed_key {
        Event::Key(key) => {
            if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
                panic!("Program end");
            } else {
                Ok(())
            }
        }
        _ => Ok(()),
    }
}

const fn die(error_message: &str) {
    panic!("{}", error_message);
}
