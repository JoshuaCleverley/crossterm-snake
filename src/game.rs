use std::{
  io::{self, Write},
  time::Duration,
};

use crossterm::{
  cursor,
  event::{poll, KeyCode, KeyEvent},
  execute,
  terminal::{self, disable_raw_mode, enable_raw_mode},
  QueueableCommand,
};
use rand::Rng;

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
        len: 0,
        tail: Vec::new(),
      },
      food: (10, 10),
      points: 0,
    }
  }

  pub fn start(&mut self) -> io::Result<()> {
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
            if self.player.dir != player::Direction::LEFT {
              self.player.dir = player::Direction::RIGHT;
            }
          }
          KeyCode::Down => {
            if self.player.dir != player::Direction::UP {
              self.player.dir = player::Direction::DOWN;
            }
          }
          KeyCode::Left => {
            if self.player.dir != player::Direction::RIGHT {
              self.player.dir = player::Direction::LEFT;
            }
          }
          KeyCode::Up => {
            if self.player.dir != player::Direction::DOWN {
              self.player.dir = player::Direction::UP;
            }
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

      let mut rng = rand::thread_rng();
      loop {
        let new_x = rng.gen_range(1..self.screen_size.0 - 1);
        let new_y = rng.gen_range(1..self.screen_size.1 - 3);
        let new_food = (new_x, new_y);

        if !self.player.tail.contains(&new_food) {
          self.food = new_food;
          break;
        }
      }

      self.player.len += 1;
    }

    if self.player.pos.0 < 1
      || self.player.pos.0 > self.screen_size.0 - 2
      || self.player.pos.1 < 1
      || self.player.pos.1 > self.screen_size.1 - 3
      || self.player.tail.contains(&self.player.pos)
    {
      self.player.len = 0;
      self.player.pos = (self.screen_size.0 / 2, self.screen_size.1 / 2);
      self.player.tail = Vec::new();
      self.points = 0;
    }

    Ok(())
  }

  fn exit(&self) -> io::Result<()> {
    execute!(&self.stdout, cursor::Show)?;
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
