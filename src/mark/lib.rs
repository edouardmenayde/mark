//! Mark
//!
//! A nano like markdown editor.
//!
//! This module contains all you need to create an `mark` executable.
extern crate gapbuffer;
extern crate unicode_width;
extern crate rustbox;

pub use editor::Editor;

mod buffer;

mod iterators {
  pub mod lines;
}

mod editor;
mod view;
mod mark;
