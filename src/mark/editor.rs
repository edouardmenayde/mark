use std::path::PathBuf;
use std::sync::{Mutex, Arc};
use std::error::Error;

use rustbox::{RustBox, Event, Key};

use view::View;
use buffer::Buffer;

pub struct Editor {
  buffers: Vec<Arc<Mutex<Buffer>>>,
  view: View,
  rustbox: RustBox,
}

impl Editor {
  pub fn new(source: PathBuf, rustbox: RustBox) -> Editor {
    let height = rustbox.height();
    let width = rustbox.width();
    let mut buffers = Vec::new();

    let buffer = Buffer::new_from_file(source);

    buffers.push(Arc::new(Mutex::new(buffer)));

    let view = View::new(buffers[0].clone(), width, height);

    Editor {
      buffers,
      view,
      rustbox
    }
  }

  fn draw(&mut self) {
    self.view.draw(&mut self.rustbox);
  }

  pub fn start(&mut self) {
    loop {
      self.draw();
      self.rustbox.present();

      match self.rustbox.poll_event(false) {
        Ok(Event::KeyEvent(key)) => {
          match key {
            Key::Char('q') => { break; }
            _ => {}
          }
        }
        Err(e) => panic!("{}", e.description()),
        _ => {}
      }
    }
  }
}

