#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
mod editor;

use editor::Editor;

fn main() {
    let editor = Editor::default();
    editor.run();
}
