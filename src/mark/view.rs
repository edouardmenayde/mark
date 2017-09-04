use std::sync::{Mutex, Arc};
use mark::Mark;
use buffer::Buffer;
use rustbox::{Color, RustBox, Style as RustBoxStyle};
use unicode_width::UnicodeWidthChar;

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

      let mut lines = buffer.get_lines().take(height);

      for y in 0..height {
        let mut x = 0;
        let line = lines.next().unwrap_or_else(Vec::new);

        for character in line.iter() {
          let char = *character as char;

          rustbox.print_char(x, y, RustBoxStyle::empty(), Color::White, Color::Black, char);

          x += UnicodeWidthChar::width(char).unwrap_or(1);
        }

        if line.len() > width {
          rustbox.print_char(width - 1, y, RustBoxStyle::empty(), Color::White, Color::Black, '→');
        }
      }
    }
  }
}

