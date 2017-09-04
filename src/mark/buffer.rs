// Standard dependencies
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

// External dependencies
use gapbuffer::GapBuffer;
use iterators::lines::Lines;

pub struct Buffer {
  pub text: GapBuffer<u8>
}

impl From<File> for Buffer {
  fn from(mut file: File) -> Buffer {
    let mut buffer = Buffer::new();
    let mut content = String::new();

    if file.read_to_string(&mut content).is_ok() {
      buffer.text.extend(content.bytes())
    }

    buffer
  }
}

impl Buffer {
  pub fn new() -> Buffer {
    Buffer {
      text: GapBuffer::new()
    }
  }

  pub fn new_from_file(path: PathBuf) -> Buffer {
    match File::open(&path) {
      Ok(file) => {
        let mut buffer = Buffer::from(file);

        buffer
      }
      Err(_) => {
        Buffer::new()
      }
    }
  }

  fn get_length(&self) -> usize {
    self.text.len() + 1
  }

  pub fn get_lines(&self) -> Lines {
    Lines {
      buffers: &self.text,
      length: self.get_length(),
      position: 0
    }
  }
}
