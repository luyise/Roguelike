pub mod colors;
pub mod game_structures;
pub mod options;

use colors::*;
use game_structures::*;
use options::*;

use std::io::stdout;
use std::time::Duration;

use crossterm::event::{Event, KeyCode};
use crossterm::style::{Color, Print, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::ClearType;
use crossterm::{cursor, event, execute, terminal, Result};

fn main() -> Result<()> {
    let mut gs = GameState::new();
    let mut screen_state = gs.make_screen_state();

    // - test, j'ajoute des obstacles
    gs.entities.push(Entity::Obstacle(Obstacle::new(10, 10)));
    gs.entities.push(Entity::Obstacle(Obstacle::new(10, 11)));
    gs.entities.push(Entity::Obstacle(Obstacle::new(14, 20)));
    // -

    let (_cols, _rows) = terminal::size()?;
    execute!(
        stdout(),
        SetBackgroundColor(BACKGROUND_CLR),
        terminal::SetSize(SCREEN_WIDTH, SCREEN_HEIGHT),
        terminal::Clear(ClearType::All),
        cursor::Hide,
        terminal::SetTitle("Roguelike")
    )?;
    print_screen_background()?;
    print_screen(&screen_state)?;

    'running: loop {
        // O. Make a screenshot of the last displayed screen
        let old_screen_state = screen_state;

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
                            look()?
                        } else {
                            gs.looking = false;
                            clean_environment()?
                        }
                    }

                    // Exit command : [esc]
                    KeyCode::Esc => {
                        execute!(stdout(), cursor::Show, terminal::Clear(ClearType::All))?;
                        break 'running;
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
    execute!(stdout(), SetForegroundColor(GROUND_CLR))?;
    let empty_strip: String = " ".repeat(64usize);
    for j in 1..(SCREEN_HEIGHT - 1) {
        execute!(stdout(), cursor::MoveTo(N_WIDTH + 2, j))?;
        execute!(stdout(), Print(&empty_strip))?
    }
    Ok(())
}

fn look() -> Result<()> {
    clean_environment()?; // Displaying squares on environment screen
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        cursor::MoveTo(N_WIDTH + 2 + 15, 1 + 4),
        Print('\u{250C}'.to_string()),
        cursor::MoveTo(N_WIDTH + 2 + 15 + 1 + 10, 1 + 4),
        Print('\u{252C}'.to_string()),
        cursor::MoveTo(N_WIDTH + 2 + 15 + 1 + 10 + 1 + 10, 1 + 4),
        Print('\u{252C}'.to_string()),
        cursor::MoveTo(N_WIDTH + 2 + 15 + 1 + 10 + 1 + 10 + 1 + 10, 1 + 4),
        Print('\u{2510}'.to_string()),
        cursor::MoveTo(N_WIDTH + 2 + 15, 1 + 4 + 1 + 4),
        Print('\u{251C}'.to_string()),
        cursor::MoveTo(N_WIDTH + 2 + 15 + 1 + 10, 1 + 4 + 1 + 4),
        Print('\u{253C}'.to_string()),
        cursor::MoveTo(N_WIDTH + 2 + 15 + 1 + 10 + 1 + 10, 1 + 4 + 1 + 4),
        Print('\u{253C}'.to_string()),
        cursor::MoveTo(N_WIDTH + 2 + 15 + 1 + 10 + 1 + 10 + 1 + 10, 1 + 4 + 1 + 4),
        Print('\u{2524}'.to_string()),
        cursor::MoveTo(N_WIDTH + 2 + 15, 1 + 4 + 1 + 4 + 1 + 4),
        Print('\u{251C}'.to_string()),
        cursor::MoveTo(N_WIDTH + 2 + 15 + 1 + 10, 1 + 4 + 1 + 4 + 1 + 4),
        Print('\u{253C}'.to_string()),
        cursor::MoveTo(N_WIDTH + 2 + 15 + 1 + 10 + 1 + 10, 1 + 4 + 1 + 4 + 1 + 4),
        Print('\u{253C}'.to_string()),
        cursor::MoveTo(
            N_WIDTH + 2 + 15 + 1 + 10 + 1 + 10 + 1 + 10,
            1 + 4 + 1 + 4 + 1 + 4
        ),
        Print('\u{2524}'.to_string()),
        cursor::MoveTo(N_WIDTH + 2 + 15, 1 + 4 + 1 + 4 + 1 + 4 + 1 + 4),
        Print('\u{2514}'.to_string()),
        cursor::MoveTo(N_WIDTH + 2 + 15 + 1 + 10, 1 + 4 + 1 + 4 + 1 + 4 + 1 + 4),
        Print('\u{2534}'.to_string()),
        cursor::MoveTo(
            N_WIDTH + 2 + 15 + 1 + 10 + 1 + 10,
            1 + 4 + 1 + 4 + 1 + 4 + 1 + 4
        ),
        Print('\u{2534}'.to_string()),
        cursor::MoveTo(
            N_WIDTH + 2 + 15 + 1 + 10 + 1 + 10 + 1 + 10,
            1 + 4 + 1 + 4 + 1 + 4 + 1 + 4
        ),
        Print('\u{2518}'.to_string()),
    )?;
    for l in 0..4 {
        for k in 0..3 {
            execute!(
                stdout(),
                cursor::MoveTo(N_WIDTH + 2 + 15 + 1 + k * (10 + 1), 1 + 4 + l * (1 + 4))
            )?;
            for _ in 0..10 {
                execute!(stdout(), Print('\u{2500}'.to_string()))?
            }
        }
    }
    for k in 0..4 {
        for l in 0..3 {
            for j in 0..4 {
                execute!(
                    stdout(),
                    cursor::MoveTo(N_WIDTH + 2 + 15 + k * (1 + 10), 1 + 4 + 1 + l * (4 + 1) + j),
                    Print('\u{2502}'.to_string())
                )?
            }
        }
    }

    Ok(())
}

fn print_screen_background() -> Result<()> {
    // Making Screen Boundaries
    execute!(
        stdout(), // Coin Haut-gauche
        cursor::MoveTo(0, 0),
        SetForegroundColor(SCREEN_BOUNDARIES_CLR),
        Print('\u{2554}'.to_string()),
    )?;
    for _ in 1..=(N_WIDTH) {
        // Bord supérieur
        execute!(stdout(), Print('\u{2550}'.to_string()),)?
    }
    execute!(stdout(), Print('\u{2566}'.to_string()),)?;
    for _ in (N_WIDTH + 2)..(SCREEN_WIDTH - 1) {
        execute!(stdout(), Print('\u{2550}'.to_string()),)?
    }
    execute!(stdout(), Print('\u{2557}'.to_string()),)?;
    for j in 1..=(N_HEIGHT) {
        // Bordures verticales
        execute!(
            stdout(),
            cursor::MoveTo(0, j),
            Print('\u{2551}'.to_string()),
            cursor::MoveTo(N_WIDTH + 1, j),
            Print('\u{2551}'.to_string()),
            cursor::MoveTo(SCREEN_WIDTH - 1, j),
            Print('\u{2551}'.to_string()),
        )?
    }
    execute!(
        stdout(),
        cursor::MoveTo(0, N_HEIGHT + 1),
        Print('\u{2560}'.to_string()),
    )?;
    execute!(
        stdout(),
        cursor::MoveTo(N_WIDTH + 1, N_HEIGHT + 1),
        Print('\u{2563}'.to_string()),
    )?;
    execute!(
        stdout(),
        cursor::MoveTo(SCREEN_WIDTH - 1, N_HEIGHT + 1),
        Print('\u{2551}'.to_string()),
    )?;
    for j in (N_HEIGHT + 2)..(SCREEN_HEIGHT - 1) {
        execute!(
            stdout(),
            cursor::MoveTo(0, j),
            Print('\u{2551}'.to_string()),
            cursor::MoveTo(N_WIDTH + 1, j),
            Print('\u{2551}'.to_string()),
            cursor::MoveTo(SCREEN_WIDTH - 1, j),
            Print('\u{2551}'.to_string()),
        )?
    }
    execute!(
        stdout(), // Bord horizontal intermédiaire
        cursor::MoveTo(1, N_HEIGHT + 1)
    )?;
    for _ in 1..=(N_WIDTH) {
        execute!(stdout(), Print('\u{2550}'.to_string()),)?
    }
    execute!(
        stdout(), // Bord horizontal inférieure
        cursor::MoveTo(0, SCREEN_HEIGHT - 1),
        Print('\u{255A}'.to_string())
    )?;
    for _ in 1..=(N_WIDTH) {
        execute!(stdout(), Print('\u{2550}'.to_string()),)?
    }
    execute!(stdout(), Print('\u{2569}'.to_string()))?;
    for _ in (N_WIDTH + 2)..(SCREEN_WIDTH - 1) {
        execute!(stdout(), Print('\u{2550}'.to_string()),)?
    }
    execute!(stdout(), Print('\u{255D}'.to_string()),)?;

    Ok(())
}
