mod player;
pub mod map;
pub mod obstacles;

use player::*;
use obstacles::*;

use crate::colors::*;
use crate::options::*;

use crossterm::style::Color;
use std::convert::TryInto;

pub struct Point {
    pub x: i16,
    pub y: i16,
}

pub struct NonPlayerCharacter {
    pos: Point,
    sprite: char,
    info: [String; 9],                  // /!\ Une information se déclare dans un tableau de 9 lignes, chacune d'au plus 20 charactère /!\ \\
}

pub struct Ground {
    pos: Point,
    info: [String; 9],
}

pub enum Entity {
    NonPlayerCharacter(NonPlayerCharacter),
    Obstacle(Obstacle),
    Ground(Ground),
}

impl Entity {
    pub fn get_char(&self) -> char {
        match self {
            Entity::NonPlayerCharacter(npc) => npc.sprite,
            Entity::Obstacle(obs) => obs.sprite,
            Entity::Ground(_) => '\u{2591}',
        }
    }

    pub fn get_clr(&self) -> Color {
        match self {
            Entity::NonPlayerCharacter(_) => PLAYER_CLR,
            Entity::Obstacle(obs) => obs.color,
            Entity::Ground(_) => GROUND_CLR,
        }
    }

    pub fn get_pos(&self) -> &Point {
        match self {
            Entity::NonPlayerCharacter(npc) => &npc.pos,
            Entity::Obstacle(obs) => &obs.pos,
            Entity::Ground(grd) => &grd.pos,
        }
    }

    pub fn get_info(&self) -> &[String; 9] {
        match self {
            Entity::NonPlayerCharacter(npc) => &npc.info,
            Entity::Obstacle(obs) => &obs.info,
            Entity::Ground(grd) => &grd.info,
        }
    }
}

pub struct GameModifications {
    pub screen_changed: [[bool; N_HEIGHT as usize]; N_WIDTH as usize],
    pub looking_changed: bool,
    pub moved_while_looking: bool
}

impl GameModifications {
    pub fn new() -> GameModifications {
        GameModifications {
            screen_changed: [[true; N_HEIGHT as usize]; N_WIDTH as usize],
            looking_changed: false,
            moved_while_looking: false
        }
    }
}

#[derive(Clone)]
pub struct ScreenState {
    pub grid: [[(char, Color); N_HEIGHT as usize]; N_WIDTH as usize],
}

pub struct GameState {
    pub player: Player,
    pub entities: Vec<Entity>,
    pub screen_pos: Point,
    pub looking: bool,

    pub modifications: GameModifications
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            player: Player::new(),
            entities: Vec::new(),
            screen_pos: Point { x: 0, y: 0 },
            looking: false,

            modifications: GameModifications::new()
        }
    }

    pub fn make_screen_state(&self) -> ScreenState {
        let mut grid = [[(' ', Color::Black); N_HEIGHT as usize]; N_WIDTH as usize];

        for entity in &self.entities {
            let pos = entity.get_pos();
            // On regarde si l'entié est située dans l'écran de jeu
            if self.screen_pos.x <= pos.x
                && pos.x < self.screen_pos.x + (N_WIDTH as i16)
                && self.screen_pos.y <= pos.y
                && pos.y < self.screen_pos.y + (N_HEIGHT as i16)
            {
                grid[(pos.x - self.screen_pos.x) as usize][(pos.y - self.screen_pos.y) as usize] =
                    (entity.get_char(), entity.get_clr())
            }
        }
        grid[(self.player.pos.x - self.screen_pos.x) as usize]
            [(self.player.pos.y - self.screen_pos.y) as usize] = (self.player.sprite, PLAYER_CLR);

        ScreenState { grid: grid }
    }

    pub fn refresh_modifications(&mut self) {
        self.modifications = GameModifications::new()
    }

    pub fn move_player(&mut self, dx: i16, dy: i16) {
        let x = self.player.pos.x;
        let y = self.player.pos.y;
        let nx: i16 = x as i16 + dx;
        let ny: i16 = y as i16 + dy;

        if nx < 0 || nx >= MAP_WIDTH as i16 || ny < 0 || ny >= MAP_HEIGHT as i16 {
            return;
        }

        for e in self.entities.iter() {
            let e_pos = e.get_pos();
            if e_pos.x as i16 == nx && e_pos.y as i16 == ny {
                return;
            }
        }

        // Si le joueur pousse contre le bord de l'écran, on scroll si c'est possible.
        if (3 <= nx && nx - 3 < self.screen_pos.x.try_into().unwrap())
            || (nx + 3 < MAP_WIDTH.try_into().unwrap()
                && nx + 3 >= (self.screen_pos.x + N_WIDTH as i16).try_into().unwrap())
            || (3 <= ny && ny - 3 < self.screen_pos.y.try_into().unwrap())
            || (ny + 3 < MAP_HEIGHT.try_into().unwrap()
                && ny + 3 >= (self.screen_pos.y + N_HEIGHT as i16).try_into().unwrap())
        {
            self.screen_pos.x = (self.screen_pos.x as i16 + dx).try_into().unwrap();
            self.screen_pos.y = (self.screen_pos.y as i16 + dy).try_into().unwrap()
        };

        self.player.pos.x = nx.try_into().unwrap();
        self.player.pos.y = ny.try_into().unwrap();

        if self.looking {
            self.modifications.moved_while_looking = true
        }
    }
}
