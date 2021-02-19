use crate::terminal::Terminal;
use termion::event::Key;
use std::io::{self};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Position {
  pub x: usize,
  pub y: usize
}

pub struct Editor {
  should_quit: bool,
  terminal: Terminal,
  position: Position
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
        match pressed_key {
          Key::Ctrl('q') => self.should_quit = true,
          Key::Up | Key::Down | Key::Left | Key::Right |
          Key::PageUp | Key::PageDown | Key::Home | Key::End=> self.move_cursor(pressed_key),
          _ => (),
        }

        Ok(())
    }

    fn move_cursor(&mut self, key: Key) {
      let Position{mut x, mut y} = self.position;
      let height = self.terminal.get_size().height.saturating_sub(1) as usize;
      let width = self.terminal.get_size().width.saturating_sub(1) as usize;

      match key {
        Key::Up => y = y.saturating_sub(1),
        Key::Down => {
          if y < height {
            y = y.saturating_add(1) 
          }
        },
        Key::Left => x = x.saturating_sub(1),
        Key::Right => {
          if x < width {
            x = x.saturating_add(1)
          }
        },
        Key::PageUp => y = 0,
        Key::PageDown => y = height,
        Key::Home => x = 0,
        Key::End => x = width,
        _ => ()
      }
      self.position = Position{x, y};
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error>{
      Terminal::hide_cursor();
      Terminal::go_to(&Position{x: 0, y: 0});
      if self.should_quit {
        Terminal::clear();
        println!("Goodbye \r");
      } else {
        self.draw_pipe_rows();
        Terminal::go_to(&self.position);
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
          terminal: Terminal::default().expect("Failed to start terminal"),
          position: Position{x: 0, y: 0}
        }
    }
}


fn die(err: io::Error) {
  Terminal::clear();
  panic!(err)
}
