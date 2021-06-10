// https://stackoverflow.com/questions/22273046/how-to-print-the-extended-ascii-code-in-java-from-integer-value/22274036
// https://jonasjacek.github.io/colors/

use std::time::Duration;
use std::io::stdout;

use crossterm::{event, cursor, terminal, execute, Result};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::ClearType;
use crossterm::style::{Print, SetForegroundColor, SetBackgroundColor, Color};

const BACKGROUND_CLR: Color = Color::Black;
const PLAYER_CLR: Color = Color::White;
const SCREEN_BOUNDARIES_CLR: Color = Color::AnsiValue(215);  // SandyBrown
const GROUND_CLR: Color = Color::AnsiValue(242); // Grey42

const N_WIDTH: u16 = 64;
const N_HEIGHT: u16 = 24;

fn main() -> Result<()> {
    
    let mut xpos = 3u16;
    let mut ypos = 3u16;

    let (_cols, _rows) = terminal::size()?;
    execute!(stdout(),
        cursor::Hide,
        SetBackgroundColor(BACKGROUND_CLR),
        terminal::Clear(ClearType::All)
    )?;
    print_screen_background()?;

    'running: loop {
        //  I. Handle events
        if event::poll(Duration::from_millis(500)).unwrap() {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            match event::read().unwrap() {
                Event::Key(event) => match event.code {
                    KeyCode::Left => {
                        disp('\u{2591}', xpos, ypos, GROUND_CLR)?; // Il faut changer le rendering de place /!\
                        if xpos > 1 {xpos -= 1}
                    },
                    KeyCode::Right => {
                        disp('\u{2591}', xpos, ypos, GROUND_CLR)?; // Il faut changer le rendering de place /!\
                        if xpos < N_WIDTH - 2 {xpos += 1}
                    },
                    KeyCode::Up => {
                        disp('\u{2591}', xpos, ypos, GROUND_CLR)?; // Il faut changer le rendering de place /!\
                        if ypos > 1 {ypos -= 1}
                    },
                    KeyCode::Down => {
                        disp('\u{2591}', xpos, ypos, GROUND_CLR)?; // Il faut changer le rendering de place /!\
                        if ypos < N_HEIGHT - 2 {ypos += 1}
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

        // III. Render
        disp('@', xpos, ypos, PLAYER_CLR)?;

        // IV. Time management
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60))

    };
    #[allow(unreachable_code)]
    Ok(())
}

fn disp(c : char, i: u16, j: u16, clr: Color) -> Result<()> {
    execute!(stdout(),
        cursor::MoveTo(i, j),
        SetForegroundColor(clr),
        Print(c.to_string()),
    )
}

fn print_screen_background() -> Result<()> {

    // Making Screen Boundaries
    execute!(stdout(),
        cursor::MoveTo(0, 0),
        SetForegroundColor(SCREEN_BOUNDARIES_CLR),
        Print('\u{2554}'.to_string()),
    )?;
    for _ in 1..(N_WIDTH-1) {
        execute!(stdout(),
            Print('\u{2550}'.to_string()),
        )?
    };
    execute!(stdout(),
            Print('\u{2557}'.to_string()),
    )?;
    for j in 1..(N_HEIGHT-1) {
        execute!(stdout(),
            cursor::MoveTo(0, j),
            Print('\u{2551}'.to_string()),
            cursor::MoveTo(N_WIDTH-1, j),
            Print('\u{2551}'.to_string()),
        )?
    };
    execute!(stdout(),
        cursor::MoveTo(0, N_HEIGHT-1),
        Print('\u{255A}'.to_string()),
    )?;
    for _ in 1..(N_WIDTH-1) {
        execute!(stdout(),
            Print('\u{2550}'.to_string()),
        )?
    };
    execute!(stdout(),
        Print('\u{255D}'.to_string()),
    )?;

    // Making Ground
    execute!(stdout(), SetForegroundColor(GROUND_CLR))?;
    let ground_strip: String = "\u{2591}".repeat((N_WIDTH-2) as usize);
    for j in 1..N_HEIGHT-1 {
        execute!(stdout(), cursor::MoveTo(1, j))?;
        execute!(stdout(),
            Print(&ground_strip)
        )?  
    };

    Ok(())
}