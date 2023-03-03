// importus
use std::{vec, io::stdout};

use Adventure_game::game::Character;
use  Adventure_game::game::items::{Sword,Armour, BonusDamage,BonusProtection};
use Adventure_game::game::enemy::Faction;
use Adventure_game::game::Game;
use Adventure_game::option_selector::chooseCharacter;
use crossterm::{
    event::{DisableMouseCapture, EventStream},
};
use termion::raw::IntoRawMode;



fn main() { 

    let characters: Vec<Character> = vec![
        Character{
            name: String::from("Gyulameleg"),
            health: 3.0,

            weapon:  Sword { 
                attack: 2.0, 
                bonus_dmg: BonusDamage {
                        faction: Faction::Void,
                        amount: 2.0,
                    }
             },
            armour: Armour { 
                protection: 2.0, 
                bonus_protection: BonusProtection { 
                    faction: Faction::Skeleton, 
                    amount: 5.0 
                 }
              }
        },
        Character{
            name: String::from("ziak"),
            health: 6.0,

            weapon:  Sword { 
                attack: 1.0, 
                bonus_dmg: BonusDamage {
                        faction: Faction::Void,
                        amount: 5.0,
                    }
             },
            armour: Armour { 
                protection: 10.0, 
                bonus_protection: BonusProtection { 
                    faction: Faction::Skeleton, 
                    amount: 5.0 
                 }
              }
        },
    ];
    
    // player selecting from characters
    let choosen_character:usize = async_std::task::block_on(chooseCharacter( characters.clone() ));

    let mut game: Game = Game::new(characters[choosen_character].to_owned());

    let mut stdout = stdout().into_raw_mode().unwrap();
    

    // where the game runs
    loop {

        game.generate_enemy();
        game.announce_enemy(&mut stdout);
        
        //this blocks/halts the whole program, and everyting  inside this function is async
        let outcome =         async_std::task::block_on( game.fight_enemy() );     
        
        if !outcome {
            game.announce_death(&mut stdout);
            break;
        }

        game.xp += game.enemy.xp;

    }
}


/*
https://piped.lunar.icu/watch?v=VAo065vRO4Q&t=102

My mother told me
Some day I wil buy (buy)
Galley with good oears
Sail to distans shores!

 */