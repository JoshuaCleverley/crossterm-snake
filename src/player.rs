use std::io::{self, Stdout};

use crate::graphics;

#[derive(PartialEq)]
pub enum Direction {
  RIGHT,
  DOWN,
  LEFT,
  UP,
}

pub struct Player {
  pub pos: (u16, u16),
  pub dir: Direction,
  pub len: usize,
  pub tail: Vec<(u16, u16)>,
}

impl Player {
  pub fn update(&mut self) -> io::Result<()> {
    self.tail.push(self.pos);
    if self.tail.len() > self.len {
      self.tail.remove(0);
    }

    match self.dir {
      Direction::RIGHT => self.pos = (self.pos.0 + 1, self.pos.1),
      Direction::DOWN => self.pos = (self.pos.0, self.pos.1 + 1),
      Direction::LEFT => self.pos = (self.pos.0 - 1, self.pos.1),
      Direction::UP => self.pos = (self.pos.0, self.pos.1 - 1),
    }

    Ok(())
  }

  pub fn render(&mut self, stdout: &mut Stdout) -> io::Result<()> {
    graphics::draw_char(stdout, (self.pos.0, self.pos.1), b"#")?;
    self.tail.clone().into_iter().for_each(|tail_segment| {
      graphics::draw_char(stdout, tail_segment, b"~").unwrap();
    });

    Ok(())
  }
}
