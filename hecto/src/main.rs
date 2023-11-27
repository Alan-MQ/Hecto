#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;

use editor::Editor;
pub use terminal::Terminal;



fn main() {
    println!("Welcome to Hecto Text editor");
    let mut editor = Editor::default();
    editor.run();
}

