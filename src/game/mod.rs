pub mod enemy;
pub mod items;
pub mod inventory;


use std::{vec, io::{stdout, Write, Stdout}, thread, time, collections::HashMap, num::NonZeroI128, };
use crossterm::event::EventStream;
use rand::{Rng, distributions::Alphanumeric};
use termion::{raw::{IntoRawMode, RawTerminal}, color};

use crate::tui::Hitbar;

use self::{enemy::{Enemy, Faction, Reward, RewardType}, items::{Bonus, Sword, Armour}, inventory::Inventory};



#[derive(Debug, Clone)]
pub struct  Character {
    pub name: String,
    
    pub armour: Armour,

    pub weapon: Sword,
    
    pub health: f64,

    pub maxhealth: f64,
    
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

    // current enemy
    pub enemy: Enemy,
    // level (increase by every kill)
    pub level: i128,

    // uuid, data
    pub inventory: Inventory

}

impl Game {
    // creates game object
    // at init
    pub fn new(chosen: Character) -> Game {
    
        return Game {
            character: chosen,
            round: 0,
            xp: 1.0,
            level:0,
            inventory: Inventory::new(),

            enemy: Enemy { 
                name: String::from("skelly"), 
                faction: Faction::Skeleton, 
                health: 2.0, 
                damage: 2.0, 
                xp: 1.0, 

                reward: Reward::None

                 },


        }
    }

    pub fn generate_enemy( &mut self ){

        let factions: Vec<Faction> = vec![ 
            Faction::Skeleton,
            Faction::Flesh,
            Faction::Void
            ];

            let mut enemy = Enemy { 
                // TODO cool name generator like WariZkorzok or idk...
                name: rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(7)
                .map(char::from)
                .collect(),
                faction: factions[rand::thread_rng().gen_range(0..factions.len())].to_owned(),
            
                health: rand::thread_rng().gen_range(
                    self.character.health 
                    ..
                    self.character.health + (self.character.health * 0.15 ) 
                    ),

                damage: rand::thread_rng().gen_range(
                    self.character.weapon.normal
                    ..
                    self.character.weapon.normal + (self.character.weapon.normal * 0.15 ) 
                    ),
        
                // 5u get 10% xp from enemy based on ur xp
                // so for 100 xp u get 10
                xp: (self.xp * 0.15), 
                reward: Reward::None,
        
        };


        // generating reward 1 in 11
        let number = rand::thread_rng().gen_range(0..7) ;
        if  number == 4 {  

        // gnerating from sword armour potion based on rng
        match rand::thread_rng().gen_range(1..3) {

            // Generating sword
            1 => {
                let mut gen_bonus: Bonus = Bonus::Zero;

                 // bonus or no bonus
                match rand::thread_rng().gen_range(0..3) {  
                    2 => {
                        gen_bonus = Bonus::Has {
                            faction: factions[rand::thread_rng().gen_range(0..factions.len())].to_owned(),
                            amount: rand::thread_rng().gen_range(
                                self.character.weapon.normal
                                ..
                                self.character.weapon.normal + (self.character.weapon.normal * 0.35 ) 
                                ),
                        };
            
                    },
                    _ => {}

                };

                // item object init
                let mut item = Reward::Item { 
                    tipus: RewardType::Sword { 
                        data: Sword { 

                            name: rand::thread_rng()
                            .sample_iter(&Alphanumeric)
                            .take(7)
                            .map(char::from)
                            .collect(),

                            normal:  rand::thread_rng().gen_range(
                                self.character.weapon.normal
                                ..
                                self.character.weapon.normal + (self.character.weapon.normal * 0.25 ) 
                                ),
                            
                            // none or bonus based on rng
                            bonus: gen_bonus, 
                         }
                     }
                };

            }// end of generating SWORD

            // generating Armour
            2 => {

                let mut gen_bonus: Bonus = Bonus::Zero;
                
                 // bonus or no bonus
                 match rand::thread_rng().gen_range(0..1) {  
                    1 => {
                        gen_bonus = Bonus::Has {
                            faction: factions[rand::thread_rng().gen_range(0..factions.len())].to_owned(),
                            amount: rand::thread_rng().gen_range(
                                self.character.armour.normal
                                ..
                                self.character.armour.normal + (self.character.armour.normal * 0.35 ) 
                                ),
                        }                        
                    }

                    // ignoring else
                    _ => {}
                };


                let mut item = Reward::Item { 
                    tipus: RewardType::Armour { 
                        data: Armour { 
                            
                            name: rand::thread_rng()
                            .sample_iter(&Alphanumeric)
                            .take(7)
                            .map(char::from)
                            .collect(),

                            normal: rand::thread_rng().gen_range(
                                self.character.weapon.normal
                                ..
                                self.character.weapon.normal + (self.character.weapon.normal * 0.25 ) 
                                ),
                            
                            bonus: gen_bonus,
                         }
                     }
                 };
            }
            _ => {}

    }


    self.enemy = enemy;

    }
}

pub fn increase_level(&mut self){
    self.level += 1;
}

pub fn missed_attack(&mut self){
    
    self.character.health -= ( self.enemy.damage - (self.enemy.damage * (self.character.armour.normal as f64 / 100.0) ) )
}


pub fn hit_attack(&mut self){
    self.enemy.health -= self.character.weapon.normal; 
}

pub fn announce_death(&self, stdout: &mut RawTerminal<Stdout>){

    let (x, y) = termion::terminal_size().unwrap();

    writeln!( stdout, 
        "{} {} {} {} {} {} {} {} {} {} {} {} {} ",
        termion::clear::All,
        
        termion::cursor::Goto(x/2, y/2 -3),
        termion::color::Bg(color::Red),
        String::from("You have died"),
        termion::color::Bg(color::Reset),

        termion::cursor::Goto(x/2, y/2),
        self.enemy.name,


        termion::cursor::Goto(x/2, y/2-2),
        String::from("Scores:"),

        termion::cursor::Goto(x/2, y/2+2),
        format!("Levels: {} {} {}", 
            termion::color::Bg(color::Blue),
            self.level,
            termion::color::Bg(color::Reset) ),

        termion::cursor::Goto(x/2, y/2+3),
        format!("XP: {} {} {}", 
            termion::color::Bg(color::Blue),
            self.xp.round(),
            termion::color::Bg(color::Reset) )
        
         ).expect("error printing")

}

pub fn announce_enemy(&self, stdout: &mut RawTerminal<Stdout>){

    let (x, y) = termion::terminal_size().unwrap();



    writeln!( stdout, 
        "{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
        termion::clear::All,
        
        termion::cursor::Goto(x/2, y/2 + 1),
        termion::color::Bg(color::LightCyan),
        String::from("Your enemy"),
        termion::color::Bg(color::Reset),

        termion::cursor::Goto(x/2, y/2),
        self.enemy.name,


        termion::cursor::Goto(x/2, y/2+1),
        format!("Health: {} {:.2} {}", 
            termion::color::Bg(color::Red),
            self.enemy.damage.round(),
            termion::color::Bg(color::Reset) ),


        termion::cursor::Goto(x/2, y/2+2),
        format!("Damage: {} {:.2} {}", 
            termion::color::Bg(color::Blue),
            self.enemy.damage.round(),
            termion::color::Bg(color::Reset) ),

        termion::cursor::Goto(x/2, y/2+3),
        format!("XP: {} {:.2} {}", 
            termion::color::Bg(color::Blue),
            self.enemy.xp.round(),
            termion::color::Bg(color::Reset) ),

        termion::cursor::Goto(x/2, y/2+4),
        format!("Faction: {} {:?} {}", 
            termion::color::Bg(color::Blue),
            self.enemy.faction,
            termion::color::Bg(color::Reset) ),

            termion::cursor::Goto(x/2, y/2+5),
            format!("Reward: {} {:?} {}", 
                termion::color::Bg(color::Green),
                self.enemy.reward,
                termion::color::Bg(color::Reset) ),
        ).unwrap();

        thread::sleep(time::Duration::from_millis(2500));
        writeln!( stdout, "{}", termion::clear::All).expect("failed to clear console");

}

pub async fn fight_enemy(&mut self) -> bool{


    // let obstacle = rand::thread_rng().gen_range(0..3);
    let obstacle = 0;


    match obstacle {
        // hitmarker
        0 => {
            let stdout = stdout().into_raw_mode().unwrap();
            let reader = EventStream::new();

            let  mut hitbar = Hitbar::new(reader,stdout,self.to_owned());
        
            self.increase_level();

            return hitbar.play().await;

        },

        // typing challange
        1 =>{
            return false
        }
        _ => {}
        
        };
            return false;
}


pub fn level_up( &mut self ){
    // level up
}

pub fn enemyKill( &mut self ){

    // if enemy had reward
    if self.enemy.reward != Reward::None {

    }

}
}



