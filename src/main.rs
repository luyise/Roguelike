// https://stackoverflow.com/questions/22273046/how-to-print-the-extended-ascii-code-in-java-from-integer-value/22274036
// https://jonasjacek.github.io/colors/

use std::time::Duration;
use std::io::stdout;
use std::convert::TryInto;

use crossterm::{event, cursor, terminal, execute, Result};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::ClearType;
use crossterm::style::{Print, SetForegroundColor, SetBackgroundColor, Color};

const BACKGROUND_CLR: Color = Color::Black;
const PLAYER_CLR: Color = Color::White;
const SCREEN_BOUNDARIES_CLR: Color = Color::AnsiValue(215);  // SandyBrown
const GROUND_CLR: Color = Color::AnsiValue(242); // Grey42

const SCREEN_WIDTH: u16 = 66;
const SCREEN_HEIGHT: u16 = 26;
const N_WIDTH: u16 = 64;
const N_HEIGHT: u16 = 24;
const MAP_WIDTH: u16 = 64;
const MAP_HEIGHT: u16 = 24;

struct Point {
    x: u16,
    y: u16
}

struct Player {
    pos: Point,
    sprite: char,
}
impl Player {
    fn new() -> Player {
        Player {
            pos: Point { x: 0, y: 0 },
            sprite: '@'
        }   
    }
}

struct NonPlayerCharacter {
    pos: Point,
    sprite: char,
}

struct Obstacle {
    pos: Point,
    sprite: char,
}
impl Obstacle {
    fn new(i: u16, j: u16) -> Obstacle {
        Obstacle {
            pos: Point { x: i, y: j },
            sprite: '\u{25A0}'
        }   
    }
}

struct Ground {
    pos: Point
}

enum Entity {
    NonPlayerCharacter(NonPlayerCharacter),
    Obstacle(Obstacle),
    Ground(Ground)
}
impl Entity {

    fn get_char(&self) -> char {
        match self {
            Entity::NonPlayerCharacter(npc) => npc.sprite,
            Entity::Obstacle(obs) => obs.sprite,
            Entity::Ground(_) => '\u{2591}'
        }
    }

    fn get_clr(&self) -> Color {
        match self {
            Entity::NonPlayerCharacter(_) => PLAYER_CLR,
            Entity::Obstacle(_) => SCREEN_BOUNDARIES_CLR,
            Entity::Ground(_) => GROUND_CLR
        }
    }

    fn get_pos(&self) -> &Point {
        match self {
            Entity::NonPlayerCharacter(npc) => &npc.pos,
            Entity::Obstacle(obs) => &obs.pos,
            Entity::Ground(grd) => &grd.pos
        }
    }
}

#[derive(Clone)]
struct ScreenState {
    grid: [[(char, Color); N_HEIGHT as usize]; N_WIDTH as usize],
}

struct GameState {
    player: Player,
    entities: Vec<Entity>,
}
impl GameState {
    fn new() -> GameState { 
        GameState {
            player: Player::new(),
            entities: Vec::new()
        }
    }

    fn make_screen_state(&self) -> ScreenState {

        let mut grid = [[(' ', Color::Black); N_HEIGHT as usize]; N_WIDTH as usize];

        for entity in &self.entities {
            let pos = entity.get_pos();
            // Ici on peut ajouter une condition pour tester si le point est dans l'écran ou non
            grid[pos.x as usize][pos.y as usize] = (entity.get_char(), entity.get_clr())
        };
        grid[self.player.pos.x as usize][self.player.pos.y as usize] = ('@', PLAYER_CLR);

        ScreenState { grid: grid }
    }

    fn move_player(&mut self, dx: i16, dy: i16) {
        let x = self.player.pos.x;
        let y = self.player.pos.y;
        let nx: u16 = (x as i16 + dx).try_into().unwrap();
        let ny: u16 = (y as i16 + dy).try_into().unwrap();

        // Plus tard, il faudra remplcer ça par du scrolling
        if nx >= 0 && nx < N_WIDTH && ny >= 0 && ny < N_HEIGHT {
            
            for e in self.entities.iter() {
                let e_pos = e.get_pos();
                let e_x = e_pos.x;
                let e_y = e_pos.y;
                if e_x == nx && e_y == ny {
                    return ()
                }
            }
            self.player.pos.x = nx;
            self.player.pos.y = ny
            
        }

    }
}

fn main() -> Result<()> {
    
    let mut gs = GameState::new();
    let mut screen_state = gs.make_screen_state();

    // - test, j'ajoute des obstacles
    gs.entities.push(Entity::Obstacle(Obstacle::new(10,10)));
    gs.entities.push(Entity::Obstacle(Obstacle::new(10,11)));
    gs.entities.push(Entity::Obstacle(Obstacle::new(14,20)));
    // -

    let (_cols, _rows) = terminal::size()?;
    execute!(stdout(),
        cursor::Hide,
        SetBackgroundColor(BACKGROUND_CLR),
        terminal::Clear(ClearType::All)
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
                    KeyCode::Left | KeyCode::Char('q') => {
                        gs.move_player(-1, 0)
                    },
                    KeyCode::Right | KeyCode::Char('d') => {
                        gs.move_player(1, 0)
                    },
                    KeyCode::Up | KeyCode::Char('z') => {
                        gs.move_player(0, -1)
                    },
                    KeyCode::Down | KeyCode::Char('s') => {
                        gs.move_player(0, 1)
                    },
                    KeyCode::Esc => { 
                        execute!(stdout(),
                            cursor::Show,
                            terminal::Clear(ClearType::All)
                        )?;
                        break 'running },
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
        execute!(stdout(),
            cursor::MoveTo(0, SCREEN_HEIGHT),
            SetForegroundColor(Color::White),
            Print("This is an information!")
        )?;
        // println!("{} {}", gs.player.pos.x, gs.player.pos.y);

        // IV. Time management
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60))

    };
    #[allow(unreachable_code)]
    Ok(())
}

fn disp(c : char, i: u16, j: u16, clr: Color) -> Result<()> {
    execute!(stdout(),
        cursor::MoveTo(i+1, j+1),
        SetForegroundColor(clr),
        Print(c.to_string()),
    )
}

fn print_screen(screen_state : &ScreenState) -> Result<()> {
    for j in 0..N_HEIGHT {
        for i in 0..N_WIDTH {
            disp(screen_state.grid[i as usize][j as usize].0, i, j, screen_state.grid[i as usize][j as usize].1)?
        }
    };
    Ok (())
}

fn refresh_screen(screen_state_already_displayed: ScreenState, screen_state_to_display : &ScreenState) -> Result<()> {
    for j in 0..N_HEIGHT {
        for i in 0..N_WIDTH {
            if screen_state_already_displayed.grid[i as usize][j as usize].0 != screen_state_to_display.grid[i as usize][j as usize].0 {
                disp(screen_state_to_display.grid[i as usize][j as usize].0, i, j, screen_state_to_display.grid[i as usize][j as usize].1)?
            }
        }
    };
    Ok (())
}

fn print_screen_background() -> Result<()> {

    // Making Screen Boundaries
    execute!(stdout(),
        cursor::MoveTo(0, 0),
        SetForegroundColor(SCREEN_BOUNDARIES_CLR),
        Print('\u{2554}'.to_string()),
    )?;
    for _ in 1..(SCREEN_WIDTH-1) {
        execute!(stdout(),
            Print('\u{2550}'.to_string()),
        )?
    };
    execute!(stdout(),
            Print('\u{2557}'.to_string()),
    )?;
    for j in 1..(SCREEN_HEIGHT-1) {
        execute!(stdout(),
            cursor::MoveTo(0, j),
            Print('\u{2551}'.to_string()),
            cursor::MoveTo(SCREEN_WIDTH-1, j),
            Print('\u{2551}'.to_string()),
        )?
    };
    execute!(stdout(),
        cursor::MoveTo(0, SCREEN_HEIGHT-1),
        Print('\u{255A}'.to_string()),
    )?;
    for _ in 1..(SCREEN_WIDTH-1) {
        execute!(stdout(),
            Print('\u{2550}'.to_string()),
        )?
    };
    execute!(stdout(),
        Print('\u{255D}'.to_string()),
    )?;

    // // Making Ground
    // execute!(stdout(), SetForegroundColor(GROUND_CLR))?;
    // let ground_strip: String = "\u{2591}".repeat((SCREEN_WIDTH-2) as usize);
    // for j in 1..SCREEN_HEIGHT-1 {
    //     execute!(stdout(), cursor::MoveTo(1, j))?;
    //     execute!(stdout(),
    //         Print(&ground_strip)
    //     )?  
    // };

    Ok(())
}