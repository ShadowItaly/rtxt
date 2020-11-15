use io::Write;
use crossterm::{
    terminal,
    ExecutableCommand, 
    event::read,
    style::Print,
    event::Event,
    event::KeyEvent,
    event::KeyCode,
    cursor::MoveTo,
    cursor::MoveLeft,
    cursor::MoveToNextLine,
    execute,
};

use std::io;

/// Contains the structure for an rust text app, will contain window and buffer lists and handles the terminal setup
pub struct RtxtApp {
}

impl RtxtApp {
    /// Creates a new instance of the app and initialises the terminal.
    pub fn new() -> RtxtApp {
        let mut stdout = io::stdout();
        stdout.execute(terminal::EnterAlternateScreen).expect("Could not enter alternate screen mode in terminal.");
        terminal::enable_raw_mode().expect("Could not enable raw mode for the terminal!");
        stdout.execute(MoveTo(0,0)).unwrap();
        RtxtApp{}
    }

    ///Just echoes the keys pressed used to test raw mode.
    pub fn echo_keyboard(&mut self) {
        let mut stdout = io::stdout();
        loop {
            match read().unwrap() {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: _,
                    //clearing the screen and printing our message
                }) => break,
                Event::Key(KeyEvent {
                    code: KeyCode::Backspace,
                    modifiers: _,
                    //clearing the screen and printing our message
                }) => execute!(stdout,MoveLeft(1),Print(" "),MoveLeft(1)).unwrap(),
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: _,
                    //clearing the screen and printing our message
                }) => execute!(stdout,MoveToNextLine(1)).unwrap(),
                Event::Key(KeyEvent {
                    code: KeyCode::Char(y),
                    modifiers: _,
                    //clearing the screen and printing our message
                }) => execute!(stdout,Print(y)).unwrap(),
                _ => execute!(stdout,Print("")).unwrap()
            };
        }
    }
}

impl Drop for RtxtApp {
    /// Drops the instance of the app and resets the terminal to the original state.
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode for the terminal!");
        io::stdout().execute(terminal::LeaveAlternateScreen).expect("Could not leave alternate screen mode in terminal.");
    }
}

#[cfg(test)]
mod test {

    ///Nothing to test yet so add dummy test to not forget later on!
    #[test]
    fn basic_unit_test() {
        assert_eq!(2+2,4);
    }
}
