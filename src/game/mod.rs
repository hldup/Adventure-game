
// everthing is i128 since the game goes indefinetly and values will rise. therefore some stats will overflow i8-i64

use std::{vec, io::{stdout, Write, Stdout}, thread, time, };
use crossterm::event::EventStream;
use rand::Rng;
use termion::{raw::{IntoRawMode, RawTerminal}, color};

use crate::tui::Hitbar;
 


#[derive(Debug, Clone)]
pub struct  Character {
    pub name: String,
    pub attack: f64,
    pub health: f64,
    pub protection: f64,

}#[derive(Debug,Clone)]
pub struct PotionInventory{
    pub small_heal: i128,
    pub medium_heal: i128,
    pub large_heal: i128,
    pub small_strength: i128,
    pub large_strength: i128,
    pub invisibility: i128,
}
#[derive(Debug, Clone)]
pub enum Faction {
    Skeleton,
    Flesh,
    Void,
}
#[derive(Debug,Clone)]
pub struct Enemy {
    pub name: String,
    pub faction: Faction,
    pub health: f64,
    pub damage:  f64,
    pub xp: f64,

}
// same here, everything i128
#[derive(Debug,Clone)]
pub struct Game {
    /// character stats like attack, health and dodge are edited before attacks
    pub character: Character,
    // current round
    pub round: i128,
    // overall xp
    pub xp: f64, 
    // inventory of users potion
    pub potions: PotionInventory,
    // current enemy
    pub enemy: Enemy,
    // level (increase by every kill)
    pub level: i128,
}


impl Game {
    pub fn generate_enemy(&mut self){

        let factions: Vec<Faction> = vec![ 
            Faction::Skeleton,
            Faction::Flesh,
            Faction::Void
            ];
        
        self.enemy = Enemy { 
        
            // TODO cool name generator like WariZkorzok or idk...
            name: String::from("asd"),
            faction: factions[rand::thread_rng().gen_range(0..factions.len())].to_owned(),
        
            health: rand::thread_rng().gen_range(
                (self.character.health - (self.character.health * 0.15 ))
                ..
                (self.character.health + (self.character.health * 0.15 )) 
                ),
            damage: rand::thread_rng().gen_range(
                (self.character.attack - (self.character.attack * 0.15 ))
                ..
                (self.character.attack + (self.character.attack * 0.15 )) 
                ),

            // 5u get 10% xp from enemy based on ur xp
            // so for 100 xp u get 10
            xp: (self.xp * 0.15), 
    };

}

pub fn increase_level(&mut self){
    self.level += 1;
}

pub fn missed_attack(&mut self){
    self.character.health -= ( self.enemy.damage - (self.enemy.damage * (self.character.protection as f64 / 100.0) ) )
}


pub fn hit_attack(&mut self){
    self.enemy.health -= self.character.attack; 
}


pub fn announce_enemy(&self, stdout: &mut RawTerminal<Stdout>){

    let (x, y) = termion::terminal_size().unwrap();


    writeln!( stdout, 
        "{} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
        termion::clear::All,
        
        termion::cursor::Goto(x/2, y/2 + 1),
        termion::color::Bg(color::LightCyan),
        String::from("Your enemy"),
        termion::color::Bg(color::Reset),

        termion::cursor::Goto(x/2, y/2),
        self.enemy.name,


        termion::cursor::Goto(x/2, y/2+1),
        format!("Health: {} {} {}", 
            termion::color::Bg(color::Red),
            self.enemy.damage.round(),
            termion::color::Bg(color::Reset) ),


        termion::cursor::Goto(x/2, y/2+2),
        format!("Damage: {} {} {}", 
            termion::color::Bg(color::Blue),
            self.enemy.damage.round(),
            termion::color::Bg(color::Reset) ),

        termion::cursor::Goto(x/2, y/2+3),
        format!("XP: {} {} {}", 
            termion::color::Bg(color::Blue),
            self.enemy.xp.round(),
            termion::color::Bg(color::Reset) ),

        termion::cursor::Goto(x/2, y/2+4),
        format!("Faction: {} {:?} {}", 
            termion::color::Bg(color::Blue),
            self.enemy.faction,
            termion::color::Bg(color::Reset) ),
    

        ).unwrap();

        thread::sleep(time::Duration::from_millis(2500));
        writeln!( stdout, "{}", termion::clear::All).expect("failed to clear console");

}

pub async fn fight_enemy(self){


// let obstacle = rand::thread_rng().gen_range(0..3);
let obstacle = 0;


    match obstacle {
        // hitmarker
        0 => {


            let mut stdout = stdout().into_raw_mode().unwrap();
            let mut reader = EventStream::new();
        
            self.announce_enemy(&mut stdout);

            let  mut hitbar = Hitbar::new(reader,stdout,self);
        
            hitbar.play().await;

        },

        // typing challange
        1 =>{

        }
        /// idk
        2 =>{

        }
        _ => {

        }
}

}

}




