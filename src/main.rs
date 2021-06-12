pub mod colors;
pub mod game_structures;
pub mod options;
pub mod graphics;

use colors::*;
use game_structures::*;
use game_structures::obstacles::Obstacle;
use options::*;

use std::io::stdout;
use std::time::Duration;
use std::convert::TryInto;

use crossterm::event::{Event, KeyCode};
use crossterm::style::{Color, Print, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::ClearType;
use crossterm::{cursor, event, execute, terminal, Result};

fn main() -> Result<()> {

    let mut gs = GameState::new();
    let mut screen_state = gs.make_screen_state();

    // - test, j'ajoute des obstacles
    gs.entities.push(Entity::Obstacle(Obstacle::single(10, 10)));
    gs.entities.push(Entity::Obstacle(Obstacle::single(10, 11)));
    gs.entities.push(Entity::Obstacle(Obstacle::single(14, 20)));
    // -

    let (_cols, _rows) = terminal::size()?;
    execute!(
        stdout(),
        SetBackgroundColor(BACKGROUND_CLR),
        terminal::SetSize(SCREEN_WIDTH, SCREEN_HEIGHT),
        terminal::Clear(ClearType::All),
        cursor::Hide,
        terminal::SetTitle("Roguelike"),
    )?;
    terminal::enable_raw_mode()?;
    print_screen_background()?;
    print_screen(&screen_state)?;

    'running: loop {
        // O. keeping track of what needs to be refreshed at display time // will be replaced by the "modifications" entry of struct GameState
        let old_screen_state = screen_state;
        gs.refresh_modifications();

        //  I. Handle events
        if event::poll(Duration::from_millis(500)).unwrap() {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            match event::read().unwrap() {
                Event::Key(event) => match event.code {
                    // Deplacement
                    KeyCode::Left | KeyCode::Char('q') => gs.move_player(-1, 0),
                    KeyCode::Right | KeyCode::Char('d') => gs.move_player(1, 0),
                    KeyCode::Up | KeyCode::Char('z') => gs.move_player(0, -1),
                    KeyCode::Down | KeyCode::Char('s') => gs.move_player(0, 1),

                    // Look command : [l]
                    KeyCode::Char('l') => {
                        if !(gs.looking) {
                            gs.looking = true;
                            gs.modifications.looking_changed = true
                        } else {
                            gs.looking = false;
                            gs.modifications.looking_changed = true
                        }
                    }

                    // Exit command : [esc]
                    KeyCode::Esc => {
                        execute!(stdout(), cursor::Show, terminal::Clear(ClearType::All))?;
                        break 'running
                    }
                    _ => {}
                },
                _ => {}
            }
        } else {
            // Timeout expired and no `Event` is available
        };

        // II. Update
        screen_state = gs.make_screen_state();
        if gs.modifications.looking_changed {
            if gs.looking {
                disp_look_cases()?;
                disp_look_info(&gs)?
            } else {
                clean_environment()?
            }
        } else if gs.modifications.moved_while_looking {
            disp_look_info(&gs)?
        };

        // III. Render
        refresh_screen(old_screen_state, &screen_state)?;
        execute!(
            stdout(),
            SetForegroundColor(Color::White),
            cursor::MoveTo(1, N_HEIGHT + 2),
        )?;
        println!("This is an information! ");
        execute!(stdout(), cursor::MoveTo(1, N_HEIGHT + 3),)?;
        println!(
            "DEBUG: player.pos : {} {}      ",
            gs.player.pos.x, gs.player.pos.y
        );
        execute!(stdout(), cursor::MoveTo(1, N_HEIGHT + 4),)?;
        println!(
            "DEBUG: screen.pos : {} {}      ",
            gs.screen_pos.x, gs.screen_pos.y
        );

        // IV. Time management
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30))
    }

    Ok(())
}

fn disp(c: char, i: u16, j: u16, clr: Color) -> Result<()> {
    execute!(
        stdout(),
        cursor::MoveTo(i + 1, j + 1),
        SetForegroundColor(clr),
        Print(c.to_string()),
    )
}

fn print_screen(screen_state: &ScreenState) -> Result<()> {
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

fn refresh_screen(
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

fn clean_environment() -> Result<()> {
    let empty_strip: String = " ".repeat(94usize);
    for j in 1..(SCREEN_HEIGHT - 1) {
        execute!(stdout(), cursor::MoveTo(N_WIDTH + 2, j))?;
        execute!(stdout(), Print(&empty_strip))?
    }
    Ok(())
}

fn disp_look_info(gs: &GameState) -> Result<()> {

    let empty_strip: String = " ".repeat(20usize);

    // On affiche les informations relatives à chaque case adjacente au personnage
    for i in (-1)..=1 as i16 {
        for j in (-1)..=1 as i16 {

            for l in 0..9 {
                execute!(
                    stdout(),
                    cursor::MoveTo((N_WIDTH as i16 + 2 + 15 + 1 + (i+1) * (20 + 1)).try_into().unwrap(), (1 + 3 + 1 + (j+1) * (9 + 1) + l).try_into().unwrap()),
                    Print(&empty_strip)
                )?
            };

            let x: i16 = gs.player.pos.x + i;
            let y: i16 = gs.player.pos.y + j;
            if x >= 0 && x < MAP_WIDTH.try_into().unwrap() && y >= 0 && y < MAP_WIDTH.try_into().unwrap() {
                match get_entity(gs, x, y) {
                    Some(e) => {
                        for l in 0..9 {
                            execute!(
                                stdout(),
                                cursor::MoveTo((N_WIDTH as i16 + 2 + 15 + 1 + (i+1) * (20 + 1)).try_into().unwrap(), (1 + 3 + 1 + (j+1) * (9 + 1) + l).try_into().unwrap()),
                                Print(&e.get_info()[l as usize])
                            )?
                        };
                    }
                    None => {}
                }

            }
        }
    }

    Ok(())
}

// Displaying squares on environment screen
fn disp_look_cases() -> Result<()> {
    
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
        Print(String::from("Press [l] again to restore environment"))
    )?;
    Ok(())
}

fn get_entity(gs: &GameState, x: i16, y: i16) -> Option<&Entity> {
    if 0 <= x && x < MAP_WIDTH.try_into().unwrap() && 0 <= y && y < SCREEN_HEIGHT.try_into().unwrap() {
        for e in gs.entities.iter() {
            let e_pos = e.get_pos();
            if e_pos.x == x && e_pos.y == y {
                return Some(&e)
            }
        }
    };

    return None
}

fn print_screen_background() -> Result<()> {
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
