#![warn(clippy::all, clippy::pedantic)]
mod row;
mod editor;
mod document;
mod terminal;

use editor::Editor;
pub use terminal::Terminal;
pub use document::Document;
pub use row::Row;


fn main() {
    println!("Welcome to Hecto Text editor");
    let mut editor = Editor::default();
    editor.run();
}

