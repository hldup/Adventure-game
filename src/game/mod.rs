pub mod enemy;
pub mod items;
pub mod inventory;


use std::{vec, io::{stdout, Write, Stdout}, thread, time, collections::HashMap, };
use crossterm::event::EventStream;
use rand::{Rng, distributions::Alphanumeric};
use termion::{raw::{IntoRawMode, RawTerminal}, color};

use crate::tui::Hitbar;

use self::{enemy::{Enemy, Faction}, items::{BonusDamage, Item, Armour, Sword}, inventory::Inventory};



#[derive(Debug, Clone)]
pub struct  Character {
    pub name: String,
    
    pub armour: Armour,

    pub weapon: Sword,
    
    pub health: f64,
    
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
    pub fn new(chosen: Character) -> Game {
    
        return Game {
            character: chosen,
            round: 0,
            xp: 1.0,
            level:0,
            
            enemy: Enemy { 
                name: String::from("skelly"), 
                faction: Faction::Skeleton, 
                health: 2.0, 
                damage: 2.0, 
                xp: 1.0, 

                reward: Item::Sword { 
                
                    data: items::Sword { 
                        attack: 3.0, 
                
                        bonus_dmg: BonusDamage {
                            faction: Faction::Flesh,
                            amount: 0.2,
                
                        } }
                 }  
            },

            inventory: Inventory::new(),

        };
    }

    pub fn generate_enemy( &mut self ){

        let factions: Vec<Faction> = vec![ 
            Faction::Skeleton,
            Faction::Flesh,
            Faction::Void
            ];
        
        self.enemy = Enemy { 
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
                self.character.weapon.attack
                ..
                self.character.weapon.attack + (self.character.weapon.attack * 0.15 ) 
                ),

            // 5u get 10% xp from enemy based on ur xp
            // so for 100 xp u get 10
            xp: (self.xp * 0.15), 


            reward: Item::Sword { 
                
                data: items::Sword { 
                    attack: 3.0, 
            
                    bonus_dmg: BonusDamage {
                        faction: Faction::Void,
                        amount: 0.2,
            
                    } }
             }  
    };

}


pub fn increase_level(&mut self){
    self.level += 1;
}

pub fn missed_attack(&mut self){
    
    self.character.health -= ( self.enemy.damage - (self.enemy.damage * (self.character.armour.protection as f64 / 100.0) ) )
}


pub fn hit_attack(&mut self){
    self.enemy.health -= self.character.weapon.attack; 
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

pub async fn fight_enemy(&mut self) -> bool{


// let obstacle = rand::thread_rng().gen_range(0..3);
let obstacle = 0;


    match obstacle {
        // hitmarker
        0 => {
            let mut stdout = stdout().into_raw_mode().unwrap();
            let mut reader = EventStream::new();

            let  mut hitbar = Hitbar::new(reader,stdout,self.to_owned());
        
            self.increase_level();

            return hitbar.play().await;

        },

        // typing challange
        1 =>{
            false
        }
        /// idk
        2 =>{
            false
        }
        _ => {
            false
        }
}

}



pub fn level_up( &mut self ){
    // level up
}


}




