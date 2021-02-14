use std::io::{self, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {
  should_quit: bool
}

impl Editor {
    // if not return type, then no arrow.
    // If not self ref passing, then its associated function.
    pub fn run(&mut self) {
        let _raw_mode_binder = io::stdout().into_raw_mode().unwrap();
        loop {
          if let Err(err) = self.refresh_screen() {
            die(err);
          }
          if let Err(err) = self.process_keys() {
            die(err);
          }
          if self.should_quit {
            break;
          }
        }
    }

    fn process_keys(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;
        // match pressed_key {
        //   Key::Ctrl('q') => panic!("Program end"),
        //   _ => (),
        // }
        if let Key::Ctrl('q') = pressed_key {
          self.should_quit = true;  
          // panic!("Program End");
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error>{
      print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
      if self.should_quit {
        println!("Goodbye \r");
      } else {
        self.draw_pipe_rows();
        print!("{}", termion::cursor::Goto(1, 1));
      }
      io::stdout().flush()
    }

    fn draw_pipe_rows(&self) {
      for _ in 0..24 {
        println!("|>\r");
      }
    }

    // Static functions, they don't work with an existing editor instance
    pub fn default() -> Self {
        Editor {should_quit: false}
    }
}

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        // Returns an Result, inside an option
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}

fn die(err: io::Error) {
  print!("{}", termion::clear::All);
  panic!(err)
}
