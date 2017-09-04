use gapbuffer::GapBuffer;

pub struct Lines<'a> {
  pub buffers: &'a GapBuffer<u8>,
  pub length: usize,
  pub position: usize,
}

impl<'a> Iterator for Lines<'a> {
  type Item = Vec<u8>;

  fn next(&mut self) -> Option<Vec<u8>> {
    if self.position == self.length {
      return None;
    }

    let old_position = self.position;

    self.position = (old_position..self.length)
        .filter(|i| {
          i + 1 == self.length || self.buffers[*i] == b'\n'
        })
        .take(1)
        .next()
        .unwrap() + 1;

    Some(
      (old_position..if self.position == self.length { self.position - 1 } else { self.position })
          .map(|i| self.buffers[i])
          .collect()
    )
  }
}
