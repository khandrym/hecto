use std::io::{self, Write};

use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyModifiers},
    execute, terminal,
};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    /// Function to run editor.
    pub fn run(&mut self) {
        let _enable_raw_mode_result = terminal::enable_raw_mode();

        loop {
            if let Err(error) = refresh_screen() {
                die(&error.to_string());
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(&error.to_string());
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

fn refresh_screen() -> Result<(), std::io::Error> {
    execute!(
        io::stdout(),
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;
    io::stdout().flush()
}

const fn die(error_message: &str) {
    panic!("{}", error_message);
}
