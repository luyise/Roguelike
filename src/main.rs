pub mod colors;
pub mod display;
pub mod game_structures;
pub mod graphics;
pub mod nasty_array;
pub mod options;
pub mod level_generators;

use colors::*;
use display::*;
use game_structures::{map, GameState};
use options::*;

use std::fs::File;
use std::io::stdout;
use std::time::Duration;

use crossterm::event::{Event, KeyCode};
use crossterm::style::{Color, SetBackgroundColor};
use crossterm::terminal::ClearType;
use crossterm::{cursor, event, execute, terminal, Result};

const DISPLAY_CAVERN_GENERATOR: bool = false;

fn main() -> Result<()> {
    let seed = 0u64;
    let cavern_width = 256u16;
    let cavern_height = 96u16;
    let p_filled: f64 = 0.52;
    let nb_iterations: u32 = 15;
    
    let (grid, claws, sd_grid, start_x, start_y) = crate::level_generators::cavern_generator::generate_cavern(cavern_width as usize, cavern_height as usize, seed, p_filled, nb_iterations);

    if DISPLAY_CAVERN_GENERATOR { 
        
        let name_extension: &str = "rules_0_with_filling_and_cutting";
        crate::level_generators::cavern_generator::display::display_grid( cavern_width, cavern_height, seed, p_filled, nb_iterations, name_extension, grid, claws, sd_grid)?;

        Ok(())
    } else {

    let map = map::Map::from_tile(sd_grid);

    let mut gs = GameState::new(map, start_x, start_y);
    let mut screen_state = gs.make_screen_state();

/*
    // - test, j'ajoute des obstacles
    gs.set_element_on_map(10, 10, map::obstacle::Obstacle::new().to_box())
        .unwrap();
    gs.set_element_on_map(10, 11, map::obstacle::Obstacle::new().to_box())
        .unwrap();
    gs.set_element_on_map(2, 2, map::walls::Wall::new("_SE_").to_box())
        .unwrap();
    for i in 3..24 {
        gs.set_element_on_map(i, 2, map::walls::Wall::new("__EW").to_box())
            .unwrap();
        gs.set_element_on_map(i, 30, map::walls::Wall::new("__EW").to_box())
            .unwrap();
    }
    gs.entities.push(
        Entity::NonPlayerCharacter(
            NonPlayerCharacter::new(
                Point {
                    x: 12,
                    y: 12,
                },
                'P',
                [
                    String::from(" A random peasant   "),
                    String::from("passing by.         "),
                    String::from("                    "),
                    String::from("                    "),
                    String::from("                    "),
                    String::from("                    "),
                    String::from("                    "),
                    String::from("                    "),
                    String::from("                    "),
                ],
            )
        )
    );

    gs.set_element_on_map(24, 2, map::walls::Wall::new("_S_W").to_box())
        .unwrap();
    for j in 3..30 {
        if j != 10 {
            gs.set_element_on_map(24, j, map::walls::Wall::new("NS__").to_box())
                .unwrap();
        };
        gs.set_element_on_map(2, j, map::walls::Wall::new("NS__").to_box())
            .unwrap();
    }

    gs.set_element_on_map(24, 30, map::walls::Wall::new("N__W").to_box())
        .unwrap();
    gs.set_element_on_map(2, 30, map::walls::Wall::new("N_E_").to_box())
        .unwrap();
    // -
*/

    // - test, j'ajoute un log
    gs.push_log(String::from("Press [l] to open the look table"), Color::Cyan);
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
    disp_logs(&gs)?;

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

                    KeyCode::Char('i') => gs.interact(-1, -1),
                    KeyCode::Char('k') => gs.interact(-1, 0),
                    KeyCode::Char(';') => gs.interact(-1, 1),

                    KeyCode::Char('o') => gs.interact(0, -1),
                    KeyCode::Char('f') => gs.interact(0, 0),
                    KeyCode::Char(':') => gs.interact(0, 1),

                    KeyCode::Char('p') => gs.interact(1, -1),
                    KeyCode::Char('m') => gs.interact(1, 0),
                    KeyCode::Char('=') | KeyCode::Char('!') => gs.interact(1, 1),

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
        refresh_logs(&gs)?;
        refresh_environment(&gs)?;

        // IV. Time management
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30))
    }

    let mut f = File::create("saved.txt").unwrap();
    gs.save(&mut f).unwrap();

    Ok(())

    }
}
