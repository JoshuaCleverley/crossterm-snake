use std::io::{self, Stdout};

use crate::graphics;

pub enum Direction {
  RIGHT,
  DOWN,
  LEFT,
  UP,
}

pub struct Player {
  pub pos: (u16, u16),
  pub dir: Direction,
}

impl Player {
  pub fn update(&mut self) -> io::Result<()> {
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
    Ok(())
  }
}
