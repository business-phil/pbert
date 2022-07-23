use std::io;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};

use pbert::GameBoard;

fn main() -> Result<()> {
    let mut board = GameBoard::new();

    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout)?;

    if let Err(e) = pbert::run(&mut board) {
        println!("Error: {:?}\r", e);
    }

    disable_raw_mode()
}
