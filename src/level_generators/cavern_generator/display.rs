use crate::colors::*;
use crate::options::*;

use std::io::stdout;

use crossterm::style::{Color, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::ClearType;
use crossterm::{cursor, execute, terminal, Result};
use crossterm::style::Print;

pub fn display_grid(grid: [[bool; CAVE_GENERATION_HEIGHT as usize]; CAVE_GENERATION_WIDTH as usize]) -> Result<()> {
    execute!(
        stdout(),
        SetBackgroundColor(Color::White),
        terminal::SetSize(CAVE_GENERATION_WIDTH, CAVE_GENERATION_HEIGHT),
        terminal::Clear(ClearType::All),
        cursor::Hide,
        terminal::SetTitle("Cavern_generator"),
        SetForegroundColor(Color::Black)
    )?;

    for i in 0..CAVE_GENERATION_WIDTH {
        for j in 0..CAVE_GENERATION_HEIGHT {
            let c =
                if grid[i as usize][j as usize] { '\u{2593}' } else { ' ' };
            execute!(
                stdout(),
                cursor::MoveTo(i, j),
                Print(c.to_string())
            )?
        }
    };

    Ok(())
}