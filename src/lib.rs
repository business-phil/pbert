use std::fmt;
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
    self.grid[self.token.y][self.token.x] = !self.grid[self.token.y][self.token.x];
  }

  // Y axis is flipped, so location (0, 0) is at top-left and (0, 3) is at bottom-left
  // TODO: flip y axis movement once grid prints to static location in terminal
  pub fn move_token_up(&mut self) {
    if self.token.y > 0 {
      self.token.y -= 1;
      self.flip_token_space();
    }
  }

  pub fn move_token_down(&mut self) {
    if self.token.y < self.grid.len() - 1 {
      self.token.y += 1;
      self.flip_token_space();
    }
  }

  pub fn move_token_left(&mut self) {
    if self.token.x > 0 {
      self.token.x -= 1;
      self.flip_token_space();
    }
  }

  pub fn move_token_right(&mut self) {
    if self.token.x < self.grid.len() - 1 {
      self.token.x += 1;
      self.flip_token_space();
    }
  }
}
impl fmt::Display for GameBoard {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for row in self.grid {
      for val in row {
        write!(f, "{}", if val { "0" } else { "X" })?;
      }
      write!(f, "\n\r")?;
    }
    Ok(())
  }
}

pub type Grid = [[bool; 4]; 4];

pub struct Token {
  pub x: usize,
  pub y: usize,
}

pub fn run(board: &mut GameBoard) -> Result<()> {
  board.flip_token_space();
  println!("\r{}", board);
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
        println!("{}", board);
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

  #[test]
  fn victorious_when_all_true() {
    let board = GameBoard {
      grid: [[true; 4]; 4],
      token: Token { x: 0, y: 0 },
    };

    assert!(board.is_victorious());
  }

  #[test]
  fn victorious_when_all_false() {
    let board = GameBoard {
      grid: [[false; 4]; 4],
      token: Token { x: 0, y: 0 },
    };

    assert!(board.is_victorious());
  }

  #[test]
  fn flips_value_at_token_space() {
    let mut board = GameBoard {
      grid: [[true; 4]; 4],
      token: Token { x: 1, y: 1 },
    };

    board.flip_token_space();

    assert_eq!(
      [
        [true, true, true, true],
        [true, false, true, true],
        [true, true, true, true],
        [true, true, true, true]
      ],
      board.grid
    );
  }

  #[test]
  fn moves_token() {
    let mut board = GameBoard {
      grid: [[false; 4]; 4],
      token: Token { x: 0, y: 0 },
    };

    board.move_token_right();
    assert_eq!((1, 0), (board.token.x, board.token.y));
    board.move_token_down();
    assert_eq!((1, 1), (board.token.x, board.token.y));
    board.move_token_left();
    assert_eq!((0, 1), (board.token.x, board.token.y));
    board.move_token_up();
    assert_eq!((0, 0), (board.token.x, board.token.y));
  }

  #[test]
  fn skips_movement_along_edge() {
    let mut board = GameBoard {
      grid: [[false; 4]; 4],
      token: Token { x: 0, y: 0 },
    };

    board.move_token_left();
    board.move_token_up();
    assert_eq!((0, 0), (board.token.x, board.token.y));

    board.token = Token { x: 3, y: 3 };

    board.move_token_right();
    board.move_token_down();
    assert_eq!((3, 3), (board.token.x, board.token.y));
  }
}
