#![warn(clippy::all, clippy::pedantic)]
mod editor;
use editor::Editor;

fn main() {
    println!("Papyrust running..");
    Editor::default().run(); // static functions
}
