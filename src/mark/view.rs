use std::sync::{Mutex, Arc};
use buffer::Buffer;

use rustbox::{RustBox, Color as RustBoxColor, Style as RustBoxStyle, RB_BOLD, RB_REVERSE};

use unicode_width::UnicodeWidthChar;

use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style, FONT_STYLE_BOLD, FONT_STYLE_ITALIC};

pub struct View {
  pub buffer: Arc<Mutex<Buffer>>,

  height: usize,
  width: usize,
}

impl View {
  pub fn new(buffer: Arc<Mutex<Buffer>>, width: usize, height: usize) -> View {
    View {
      buffer,
      width: width,
      height: height,
    }
  }

  fn get_width(&self) -> usize {
    self.width
  }

  fn get_height(&self) -> usize {
    self.height
  }

  /// Needs better handling in the future.
  pub fn clear(&self, rustbox: &mut RustBox) {
    rustbox.clear();
  }

  pub fn draw(&mut self, rustbox: &mut RustBox) {
    self.clear(rustbox);
    {
      let buffer = self.buffer.lock().unwrap();
      let height = self.get_height();
      let width = self.get_width();

      let ps = SyntaxSet::load_defaults_nonewlines();
      let ts = ThemeSet::load_defaults();

      let syntax = ps.find_syntax_by_extension("md").unwrap();
      let mut h = HighlightLines::new(syntax, &ts.themes["InspiredGitHub"]);

      let mut lines = buffer.get_lines().take(height);

      for y in 0..height {
        let line = lines.next().unwrap_or_else(Vec::new);
        let l = String::from_utf8(line).unwrap();
        let ranges: Vec<(Style, &str)> = h.highlight(&l);

        let mut x: usize = 0;

        for (style, text) in ranges {
          let font_style = style.font_style;

          let mut rb_style = RustBoxStyle::empty();

          if font_style.contains(FONT_STYLE_BOLD) {
            rb_style.insert(RB_BOLD);
          } else if font_style.contains(FONT_STYLE_ITALIC) {
            rb_style.insert(RB_REVERSE);
          }

          for character in text.chars() {
            match character {
              _ => {
                rustbox.print_char(x, y, rb_style, RustBoxColor::White, RustBoxColor::Black, character);

                x += UnicodeWidthChar::width(character).unwrap_or(1);
              }
            }
          }
        }

        if l.len() > width {
          rustbox.print_char(width - 1, y, RustBoxStyle::empty(), RustBoxColor::White, RustBoxColor::Black, '→');
        }
      }
    }
  }
}

