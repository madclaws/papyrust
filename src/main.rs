#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;
pub use terminal::Terminal;
use editor::Editor;

fn main() {
    println!("Papyrust running..");
    Editor::default().run(); // static functions
}
