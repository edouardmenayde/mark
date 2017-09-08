//! Mark
//!
//! A nano like markdown editor.
//!
//! This module contains all you need to create an `mark` executable.
extern crate lazy_static;
extern crate gapbuffer;
extern crate unicode_width;
extern crate rustbox;
extern crate regex;
extern crate syntect;

pub use editor::Editor;

mod buffer;

mod iterators {
  pub mod lines;
}

mod editor;
mod view;
mod mark;
