use crossterm::{self, cursor, QueueableCommand};
use std::io::{Stdout, Write};

pub const HORIZONTAL: &[u8] = b"\xE2\x94\x81";
pub const VERTICAL: &[u8] = b"\xe2\x94\x82";

pub const TOP_LEFT: &[u8] = b"\xe2\x94\x8d";
pub const TOP_RIGHT: &[u8] = b"\xe2\x94\x91";
pub const BOTTOM_LEFT: &[u8] = b"\xe2\x94\x95";
pub const BOTTOM_RIGHT: &[u8] = b"\xe2\x94\x99";

pub fn draw_char(stdout: &mut Stdout, (x, y): (u16, u16), char: &[u8]) -> std::io::Result<()> {
  stdout.queue(cursor::MoveTo(x, y))?;
  stdout.write(char)?;
  Ok(())
}

pub fn render_rect(
  stdout: &mut Stdout,
  (x, y): (u16, u16),
  (w, h): (u16, u16),
) -> std::io::Result<()> {
  // Draw horizontal lines
  for x_draw in 0..w {
    draw_char(stdout, (x + x_draw, y), HORIZONTAL)?;
    draw_char(stdout, (x + x_draw, y + h), HORIZONTAL)?;
  }
  // Draw vertical lines
  for y_draw in 0..h {
    draw_char(stdout, (x, y + y_draw), VERTICAL)?;
    draw_char(stdout, (x + w, y + y_draw), VERTICAL)?;
  }
  // Draw corners
  draw_char(stdout, (x, y), TOP_LEFT)?;
  draw_char(stdout, (x + w, y), TOP_RIGHT)?;
  draw_char(stdout, (x, y + h), BOTTOM_LEFT)?;
  draw_char(stdout, (x + w, y + h), BOTTOM_RIGHT)?;

  Ok(())
}
