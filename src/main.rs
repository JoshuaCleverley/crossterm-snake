pub mod game;
pub mod graphics;
pub mod player;

fn main() -> std::io::Result<()> {
  let mut game = game::Game::new();
  game.run()?;

  Ok(())
}
