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
    let grid: Grid = [[false; 4]; 4];
    let token = Token { x: 0, y: 0 };

    GameBoard { grid, token }
  }

  pub fn is_victorious(&self) -> bool {
    self.grid.iter().all(|&row| row.iter().all(|&v| v))
      || self.grid.iter().all(|&row| row.iter().all(|&v| !v))
  }

  pub fn flip_token_space(&mut self) {
    self.grid[self.token.x][self.token.y] = !self.grid[self.token.x][self.token.y];
  }

  pub fn move_token_up(&mut self) {
    if self.token.x > 0 {
      self.token.x -= 1;
      self.flip_token_space();
    }
  }

  pub fn move_token_down(&mut self) {
    if self.token.x < self.grid.len() - 1 {
      self.token.x += 1;
      self.flip_token_space();
    }
  }

  pub fn move_token_left(&mut self) {
    if self.token.y > 0 {
      self.token.y -= 1;
      self.flip_token_space();
    }
  }

  pub fn move_token_right(&mut self) {
    if self.token.y < self.grid.len() - 1 {
      self.token.y += 1;
      self.flip_token_space();
    }
  }
}

pub type Grid = [[bool; 4]; 4];

pub struct Token {
  pub x: usize,
  pub y: usize,
}

pub fn run(board: &mut GameBoard) -> Result<()> {
  println!("\r");
  board.flip_token_space();
  for (i, _) in board.grid.iter().enumerate() {
    println!("{:?}\r", board.grid[i])
  }
  loop {
    if poll(Duration::from_millis(1_000))? {
      let event = read()?;

      if let Event::Key(key) = event {
        match key.code {
          KeyCode::Esc => break,
          KeyCode::Up => board.move_token_up(),
          KeyCode::Down => board.move_token_down(),
          KeyCode::Left => board.move_token_left(),
          KeyCode::Right => board.move_token_right(),
          _ => {}
        }
        println!("Token location: ({}, {})\r", board.token.x, board.token.y);
        for (i, _) in board.grid.iter().enumerate() {
          println!("{:?}\r", board.grid[i])
        }
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
