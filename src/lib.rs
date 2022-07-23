use std::time::Duration;

use crossterm::{
  event::{poll, read, Event, KeyCode},
  Result,
};

pub struct GameBoard {
  pub grid: Grid,
  pub token: Token,
}
impl GameBoard {
  pub fn new() -> GameBoard {
    let grid: Grid = [false; 8];
    let token = Token { location: 0 };

    GameBoard { grid, token }
  }

  pub fn is_victorious(&self) -> bool {
    self.grid.iter().all(|&v| v)
  }

  pub fn flip_token_space(&mut self) {
    self.grid[self.token.location] = !self.grid[self.token.location];
  }

  pub fn move_token_left(&mut self) {
    if self.token.location > 0 {
      self.token.location -= 1;
      self.flip_token_space();
    }
  }

  pub fn move_token_right(&mut self) {
    if self.token.location < self.grid.len() - 1 {
      self.token.location += 1;
      self.flip_token_space();
    }
  }
}

// TODO
pub type Grid = [bool; 8];

pub struct Token {
  pub location: usize,
}

pub fn run(board: &mut GameBoard) -> Result<()> {
  println!("\r");
  board.flip_token_space();
  println!("{:?}\r", board.grid);
  loop {
    if poll(Duration::from_millis(1_000))? {
      let event = read()?;

      if let Event::Key(key) = event {
        // println!("{:?}\r", key);
        match key.code {
          KeyCode::Esc => break,
          KeyCode::Left => board.move_token_left(),
          KeyCode::Right => board.move_token_right(),
          _ => {}
        }
        println!("{:?}\r", board.grid);
        if board.is_victorious() {
          println!("VICTORY!\r");
          break;
        }
      }
    }
  }

  Ok(())
}

// TODO
#[cfg(test)]
mod tests {
  use super::*;
}
