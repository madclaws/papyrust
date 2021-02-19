use std::io::{self, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use crate::Position;

pub struct Size {
  pub width: u16,
  pub height: u16
}

pub struct Terminal {
  size: Size,
  _raw_stdout: RawTerminal<std::io::Stdout>
}

impl Terminal {
  /// Returns a terminal, with current terminal size.
  ///
  /// # Errors
  /// Error can happen on fetching terminal size.
  pub fn default() -> Result<Self, std::io::Error> {
    let size = termion::terminal_size()?;
     Ok(Self {
       size: Size {
         width: size.0,
         height: size.1
       },
       _raw_stdout: io::stdout().into_raw_mode().unwrap()
     })
  }

  #[must_use]
  pub fn get_size(&self) -> &Size {
    &self.size
  }

  /// Reads the input from stdin, and returns the key as Result.
  ///
  /// # Errors
  /// Error can happen on fetching from stdin.
  pub fn read_key() -> Result<Key, std::io::Error> {
    loop {
      // Returns an Result, inside an option
      if let Some(key) = io::stdin().lock().keys().next() {
          return key;
      }
    }
  }

  /// Flushes the stdout
  ///
  /// # Errors
  /// Error can happen on flushing stdout.
  pub fn flush() -> Result<(), std::io::Error>{
    io::stdout().flush()
  }

  pub fn clear() {
    print!("{}", termion::clear::All);
  }

  #[allow(clippy::cast_possible_truncation)]
  pub fn go_to(position: &Position) {
    let Position{x, y} = position;
    let x = x.saturating_add(1) as u16;
    let y = y.saturating_add(1) as u16;
    print!("{}", termion::cursor::Goto(x, y));
  }

  pub fn hide_cursor() {
    print!("{}", termion::cursor::Hide);
  }

  pub fn show_cursor() {
    print!("{}", termion::cursor::Show);
  }

  pub fn clear_current_line() {
    print!("{}", termion::clear::CurrentLine);
  }

}