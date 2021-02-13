use std::io::{self, Read};
use termion::raw::IntoRawMode;

fn main() {
  println!("Papyrust running..");
  
  let _raw_mode_binder = io::stdout().into_raw_mode().unwrap();
  for bytes in io::stdin().bytes() {
    
    // variable shadowing 
    
    match bytes {
      Ok(bytes) => {
        let character = bytes as char;
        if character.is_control() {
          println!("{:#b} \r", bytes);
        } else {
          println!("{:#b} => {} \r", bytes, character);
        }
        if bytes == get_control_byte('q') {
          break;
        }
      }
      Err(err) => {
        die(err);
      }
    }
  }
}

fn get_control_byte(character: char) -> u8 {
  let character = character as u8;
  character & 0b0001_1111
}

fn die(err: io::Error) {
  panic!(err)
}
