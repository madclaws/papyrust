use crate::terminal::Terminal;
use termion::event::Key;
use std::io::{self};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
  should_quit: bool,
  terminal: Terminal
}

impl Editor {
    // if not return type, then no arrow.
    // If not self ref passing, then its associated function.
    pub fn run(&mut self) {
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
        let pressed_key = Terminal::read_key()?;
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
      Terminal::hide_cursor();
      Terminal::go_to(0, 0);
      if self.should_quit {
        Terminal::clear();
        println!("Goodbye \r");
      } else {
        self.draw_pipe_rows();
        Terminal::go_to(0, 0);
      }
      Terminal::show_cursor();
      // io::stdout().flush()
      Terminal::flush()
    }

    fn draw_pipe_rows(&self) {
      let height = self.terminal.get_size().height; 
      for row in 0..height - 1 {
        Terminal::clear_current_line();
        if row == height / 3 {
          self.render_welcome_msg();
        } else {
          println!("|>\r");
        }
      }
    }

    fn render_welcome_msg(&self) {
      let mut welcome_msg = format!("PAPYRUST v{}\r", VERSION);
      let window_width: usize = self.terminal.get_size().width as usize;
      let padding = window_width / 2 - welcome_msg.len() / 2;
      let spaces = " ".repeat(padding - 1);
      welcome_msg = format!("|>{}{}", spaces, welcome_msg);
      let max_width = std::cmp::min(self.terminal.get_size().width as usize, welcome_msg.len()); 
      
      println!("{}\r", &welcome_msg[..max_width]);
    }


    // Static functions, they don't work with an existing editor instance
    pub fn default() -> Self {
        Editor {
          should_quit: false,
          terminal: Terminal::default().expect("Failed to start terminal")
        }
    }
}


fn die(err: io::Error) {
  Terminal::clear();
  panic!(err)
}
