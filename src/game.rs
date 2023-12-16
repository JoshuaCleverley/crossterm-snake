use std::{
  io::{self, Write},
  time::Duration,
};

use crossterm::{
  cursor,
  event::{poll, EnableMouseCapture, KeyCode, KeyEvent},
  execute,
  terminal::{self, disable_raw_mode, enable_raw_mode},
  QueueableCommand,
};

use crate::{
  graphics::{self, draw_char},
  player,
};

pub struct Game {
  stdout: io::Stdout,
  screen_size: (u16, u16),
  poll_duration: u64,
  player: player::Player,
  food: (u16, u16),
  points: u16,
}

impl Game {
  pub fn new() -> Game {
    Game {
      stdout: io::stdout(),
      poll_duration: 100,
      screen_size: terminal::size().unwrap(),
      player: player::Player {
        dir: player::Direction::UP,
        pos: (10, 10),
      },
      food: (10, 10),
      points: 0,
    }
  }

  pub fn start(&mut self) -> io::Result<()> {
    execute!(&self.stdout, EnableMouseCapture)?;
    execute!(&self.stdout, cursor::Hide)?;
    self.player.pos = (self.screen_size.0 / 2, self.screen_size.1 / 2);
    Ok(())
  }

  fn handle_events(&mut self) -> Result<bool, io::Error> {
    if poll(Duration::from_millis(self.poll_duration))? {
      if let crossterm::event::Event::Key(KeyEvent { code, .. }) = crossterm::event::read()? {
        match code {
          KeyCode::Esc => {
            return Ok(false);
          }
          KeyCode::Right => {
            self.player.dir = player::Direction::RIGHT;
          }
          KeyCode::Down => {
            self.player.dir = player::Direction::DOWN;
          }
          KeyCode::Left => {
            self.player.dir = player::Direction::LEFT;
          }
          KeyCode::Up => {
            self.player.dir = player::Direction::UP;
          }
          _ => {}
        }
      }
    }

    Ok(true)
  }

  fn render(&mut self) -> io::Result<()> {
    self
      .stdout
      .queue(terminal::Clear(terminal::ClearType::All))?;

    graphics::render_rect(
      &mut self.stdout,
      (0, 0),
      (self.screen_size.0, self.screen_size.1 - 2),
    )?;

    graphics::draw_char(
      &mut self.stdout,
      (0, self.screen_size.1),
      &[b"Points: ", self.points.to_string().as_bytes()].concat(),
    )?;

    self.player.render(&mut self.stdout)?;
    draw_char(&mut self.stdout, self.food, b"@")?;

    self.stdout.flush()?;
    Ok(())
  }

  fn update(&mut self) -> io::Result<()> {
    self.player.update()?;

    if self.player.pos.eq(&self.food) {
      self.points += 1;
      // Randomly move food
      // Grow player
    }

    Ok(())
  }

  fn exit(&self) -> io::Result<()> {
    execute!(&self.stdout, cursor::Show)?;
    execute!(&self.stdout, crossterm::event::DisableMouseCapture)?;
    Ok(())
  }

  pub fn run(&mut self) -> io::Result<()> {
    enable_raw_mode()?;
    self.start()?;

    loop {
      if match self.handle_events() {
        Ok(b) => !b,
        Err(_) => true,
      } {
        break;
      };

      self.update()?;
      self.render()?;
    }

    self.exit()?;
    disable_raw_mode()?;
    Ok(())
  }
}
