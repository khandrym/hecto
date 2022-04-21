use crossterm::{
    event::{read, Event, KeyCode, KeyModifiers},
    terminal,
};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    /// Function to run editor.
    pub fn run(&mut self) {
        let _enable_raw_mode_result = terminal::enable_raw_mode();

        loop {
            if let Err(error) = self.process_keypress() {
                die(&error.to_string());
            }
            if self.should_quit {
                break;
            }
        }
    }
    pub const fn default() -> Self {
        Self { should_quit: false }
    }
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = read()?;
        if let Event::Key(key) = pressed_key {
            if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
                self.should_quit = true;
            }
        }
        Ok(())
    }
}

const fn die(error_message: &str) {
    panic!("{}", error_message);
}
