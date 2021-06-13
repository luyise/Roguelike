use std::io::stdout;
use crate::options::*;
use crate::game_structures::{GameState, ScreenState};
use crate::graphics;
use crate::colors::SCREEN_BOUNDARIES_CLR;

use crossterm::style::{Color, Print, SetForegroundColor};
use crossterm::{cursor, execute, Result};

pub fn disp(c: char, i: u16, j: u16, clr: Color) -> Result<()> {
    execute!(
        stdout(),
        cursor::MoveTo(i + 1, j + 1),
        SetForegroundColor(clr),
        Print(c.to_string()),
    )
}

pub fn print_screen(screen_state: &ScreenState) -> Result<()> {
    for j in 0..N_HEIGHT {
        for i in 0..N_WIDTH {
            disp(
                screen_state.grid[i as usize][j as usize].0,
                i,
                j,
                screen_state.grid[i as usize][j as usize].1,
            )?
        }
    }
    Ok(())
}

pub fn refresh_screen(
    screen_state_already_displayed: ScreenState,
    screen_state_to_display: &ScreenState,
) -> Result<()> {
    for j in 0..N_HEIGHT {
        for i in 0..N_WIDTH {
            if screen_state_already_displayed.grid[i as usize][j as usize]
                != screen_state_to_display.grid[i as usize][j as usize]
            {
                disp(
                    screen_state_to_display.grid[i as usize][j as usize].0,
                    i,
                    j,
                    screen_state_to_display.grid[i as usize][j as usize].1,
                )?
            }
        }
    }
    Ok(())
}

pub fn clean_logs() -> Result<()> {
    let empty_strip: String = " ".repeat(N_WIDTH as usize);
    for j in (N_HEIGHT+2)..(DISP_HEIGHT + N_HEIGHT+2) {
        execute!(stdout(),
            cursor::MoveTo(1, j),
            Print(&empty_strip)
        )?
    };
    Ok(())
}

pub fn disp_logs(gs: &GameState) -> Result<()> {
    for j in 0..DISP_HEIGHT as usize {
        execute!(
            stdout(),
            cursor::MoveTo(1, N_HEIGHT+2+(j as u16)),
            SetForegroundColor(gs.logs.colors[j]),
            Print(&gs.logs.messages[j])
        )?
    };
    Ok(())
}

pub fn refresh_logs(gs : &GameState) -> Result<()> {
    if gs.modifications.logs_changed {
        clean_logs()?;
        disp_logs(gs)?;
    };
    Ok(())
}

pub fn clean_environment() -> Result<()> {
    let empty_strip: String = " ".repeat(94usize);
    for j in 1..(SCREEN_HEIGHT - 1) {
        execute!(stdout(), cursor::MoveTo(N_WIDTH + 2, j))?;
        execute!(stdout(), Print(&empty_strip))?
    }
    Ok(())
}

pub fn refresh_environment(gs : &GameState) -> Result<()> {
    if gs.modifications.looking_changed {
        if gs.looking {
            disp_look_cases()?;
            disp_look_info(&gs)?
        } else {
            clean_environment()?
        }
    } else if gs.modifications.look_data_changed {
        disp_look_info(&gs)?
    };
    Ok(())
}

pub fn disp_look_info(gs: &GameState) -> Result<()> {

    let empty_strip: String = " ".repeat(20_usize);
    let data = gs.get_info();

    for y in 0..3 {
        for x in 0..3 {
            match &data[y][x] {
                None => {
                    for l in 0..9 {
                        execute!(
                            stdout(),
                            cursor::MoveTo(N_WIDTH + 2 + 15 + 1 + (x as u16) * (20 + 1), 1 + 3 + 1 + (y as u16) * (9 + 1) + l),
                            Print(&empty_strip)
                        )?
                    }
                },
                Some(d) => {
                    for l in 0..9 {
                        execute!(
                            stdout(),
                            SetForegroundColor(Color::White),
                            cursor::MoveTo(N_WIDTH + 2 + 15 + 1 + (x as u16) * (20 + 1), 1 + 3 + 1 + (y as u16) * (9 + 1) + l),
                            Print(&d[l as usize])
                        )?
                    }
                }
            }
        }
    }

    Ok(())
}

// Displaying squares on environment screen
pub fn disp_look_cases() -> Result<()> {
    
    clean_environment()?; 

    let mut grid =  graphics::grid::Grid::new(3 * 20 + 4, 3 * 9 + 4);
    for i in 0..=3 {
        grid.draw_line(0, 10 * i, 63, 10 * i, 1).unwrap();
        grid.draw_line(21 * i, 0, 21 * i, 30, 1).unwrap()
    };
    let s = grid.to_string(graphics::grid::GridStyle::Single);
    for (i, line) in s.iter().enumerate() {
        execute!(
            stdout(),
            SetForegroundColor(Color::White),
            cursor::MoveTo(N_WIDTH + 2 + 15, 4 + i as u16),
            Print(line)
        )?
    };
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        cursor::MoveTo(N_WIDTH + 2 + 28, SCREEN_HEIGHT - 5),
        Print(String::from("Press [l] again to restore environment")),
        cursor::MoveTo(N_WIDTH + 2 + 15 + 1 + 0*21 + 9, 4 + 0*10),
        Print(String::from("[i]")),
        cursor::MoveTo(N_WIDTH + 2 + 15 + 1 + 1*21 + 9, 4 + 0*10),
        Print(String::from("[o]")),
        cursor::MoveTo(N_WIDTH + 2 + 15 + 1 + 2*21 + 9, 4 + 0*10),
        Print(String::from("[p]")),
        cursor::MoveTo(N_WIDTH + 2 + 15 + 1 + 0*21 + 9, 4 + 1*10),
        Print(String::from("[k]")),
        cursor::MoveTo(N_WIDTH + 2 + 15 + 1 + 1*21 + 9, 4 + 1*10),
        Print(String::from("[f]")),
        cursor::MoveTo(N_WIDTH + 2 + 15 + 1 + 2*21 + 9, 4 + 1*10),
        Print(String::from("[m]")),
        cursor::MoveTo(N_WIDTH + 2 + 15 + 1 + 0*21 + 9, 4 + 2*10),
        Print(String::from("[,]")),
        cursor::MoveTo(N_WIDTH + 2 + 15 + 1 + 1*21 + 9, 4 + 2*10),
        Print(String::from("[:]")),
        cursor::MoveTo(N_WIDTH + 2 + 15 + 1 + 2*21 + 7, 4 + 2*10),
        Print(String::from("[!]/[=]"))
    )?;
    Ok(())
}

pub fn print_screen_background() -> Result<()> {
    let sw = SCREEN_WIDTH as usize;
    let sh = SCREEN_HEIGHT as usize;
    let mut grid = graphics::grid::Grid::new(sw, sh);
    grid.draw_line(0, 0, sw - 1, 0, 1).unwrap();
    grid.draw_line(0, sh - 1, sw - 1, sh - 1, 1).unwrap();
    grid.draw_line(0, 0, 0, sh - 1, 1).unwrap();
    grid.draw_line(sw - 1, 0, sw - 1, sh - 1, 1).unwrap();

    grid.draw_line(1 + N_WIDTH as usize, 0, 1 + N_WIDTH as usize, sh - 1, 1).unwrap();
    grid.draw_line(0, 1 + N_HEIGHT as usize, N_WIDTH as usize, 1 + N_HEIGHT as usize, 1).unwrap();
    let s = grid.to_string(graphics::grid::GridStyle::Double);
    execute!(
        stdout(),
        SetForegroundColor(SCREEN_BOUNDARIES_CLR),
    )?;
    for (y, line) in s.iter().enumerate() {
        execute!(
            stdout(),
            cursor::MoveTo(0, y as u16),
            Print(line),
        )?;
    }

    Ok(())
}
