//! Basic text editor in Rust.

#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

mod editor;

use editor::Editor;

fn main() {
    Editor::run();
}
