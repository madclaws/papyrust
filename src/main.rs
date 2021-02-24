#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;
mod row;
mod document;
pub use terminal::Terminal;
pub use editor::Position;
pub use row::Row;
pub use document::Document;

use editor::Editor;

fn main() {
    println!("Papyrust running..");
    Editor::default().run(); // static functions
}
