use crate::colors::*;
use crate::options::*;

use crossterm::style::Color;
use std::convert::TryInto;

pub struct Point {
    pub x: u16,
    pub y: u16
}

pub struct Player {
    pub pos: Point,
    pub sprite: char,
}
impl Player {
    pub fn new() -> Player {
        Player {
            pos: Point { x: 0, y: 0 },
            sprite: '@'
        }   
    }
}

pub struct NonPlayerCharacter {
    pos: Point,
    sprite: char,
    info: [String; 4]
}

pub struct Obstacle {
    pos: Point,
    sprite: char,
    info: [String; 4]
}
impl Obstacle {
    pub fn new(i: u16, j: u16) -> Obstacle {
        Obstacle {
            pos: Point { x: i, y: j },
            sprite: '\u{25A0}',
            info: [String::new(), String::new(), String::new(), String::new()]
            // info: vec!["A U+25A0  ", "character ", "that seems", "to have   ", arrived there by mistake"]
        }   
    }
}

pub struct Ground {
    pos: Point,
    info: [String; 4]
}

pub enum Entity {
    NonPlayerCharacter(NonPlayerCharacter),
    Obstacle(Obstacle),
    Ground(Ground)
}
impl Entity {

    pub fn get_char(&self) -> char {
        match self {
            Entity::NonPlayerCharacter(npc) => npc.sprite,
            Entity::Obstacle(obs) => obs.sprite,
            Entity::Ground(_) => '\u{2591}'
        }
    }

    pub fn get_clr(&self) -> Color {
        match self {
            Entity::NonPlayerCharacter(_) => PLAYER_CLR,
            Entity::Obstacle(_) => SCREEN_BOUNDARIES_CLR,
            Entity::Ground(_) => GROUND_CLR
        }
    }

    pub fn get_pos(&self) -> &Point {
        match self {
            Entity::NonPlayerCharacter(npc) => &npc.pos,
            Entity::Obstacle(obs) => &obs.pos,
            Entity::Ground(grd) => &grd.pos
        }
    }

    pub fn get_info(&self) -> &[String; 4] {
        match self {
            Entity::NonPlayerCharacter(npc) => &npc.info,
            Entity::Obstacle(obs) => &obs.info,
            Entity::Ground(grd) => &grd.info
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
    pub looking: bool
}
impl GameState {
    pub fn new() -> GameState { 
        GameState {
            player: Player::new(),
            entities: Vec::new(),
            screen_pos: Point{ x: 0, y: 0 },
            looking: false
        }
    }

    pub fn make_screen_state(&self) -> ScreenState {

        let mut grid = [[(' ', Color::Black); N_HEIGHT as usize]; N_WIDTH as usize];

        for entity in &self.entities {
            let pos = entity.get_pos();
            // On regarde si l'entié est située dans l'écran de jeu
            if self.screen_pos.x <= pos.x && pos.x < self.screen_pos.x + SCREEN_WIDTH && self.screen_pos.y <= pos.y && pos.y < self.screen_pos.y + SCREEN_WIDTH {
                grid[(pos.x - self.screen_pos.x) as usize][(pos.y - self.screen_pos.y) as usize] = (entity.get_char(), entity.get_clr())
            }
        };
        grid[(self.player.pos.x - self.screen_pos.x) as usize][(self.player.pos.y - self.screen_pos.y) as usize] = (self.player.sprite, PLAYER_CLR);

        ScreenState { grid: grid }
    }

    pub fn move_player(&mut self, dx: i16, dy: i16) {
        let x = self.player.pos.x;
        let y = self.player.pos.y;
        let nx: i16 = x as i16 + dx;
        let ny: i16 = y as i16 + dy;

        if nx >= 0 && nx < MAP_WIDTH as i16 && ny >= 0 && ny < MAP_HEIGHT as i16 {
            
            for e in self.entities.iter() {
                let e_pos = e.get_pos();
                let e_x = e_pos.x;
                let e_y = e_pos.y;
                if e_x as i16 == nx && e_y as i16 == ny {
                    return ()
                }
            }

            // Si le joueur pousse contre le bord de l'écran, on scroll si c'est possible.
            if ( 3 <= nx && nx - 3 < self.screen_pos.x.try_into().unwrap() ) || ( nx + 3 < MAP_WIDTH.try_into().unwrap() && nx + 3 >= (self.screen_pos.x + N_WIDTH).try_into().unwrap() )
            || ( 3 <= ny && ny - 3 < self.screen_pos.y.try_into().unwrap() ) || ( ny + 3 < MAP_WIDTH.try_into().unwrap() && ny + 3 >= (self.screen_pos.y + N_HEIGHT).try_into().unwrap() ) {
                self.screen_pos.x = (self.screen_pos.x as i16 + dx).try_into().unwrap();
                self.screen_pos.y = (self.screen_pos.y as i16 + dy).try_into().unwrap()
            };
            self.player.pos.x = nx.try_into().unwrap();
            self.player.pos.y = ny.try_into().unwrap()
            
        }

    }
}