// importus
use std::{vec, io::stdout, io::Write};

use Adventure_game::game::items::{Bonus, Armour,Sword};
use Adventure_game::game::enemy::Faction;
use Adventure_game::game::{Game, Character};
use Adventure_game::option_selector::chooseCharacter;
use Adventure_game::tui::get_next_step;
use crossterm::{
    event::{DisableMouseCapture, EventStream},
};
use termion::raw::IntoRawMode;



fn main() { 

    let characters: Vec<Character> = vec![
        Character{
            name: String::from("Tank"),
            health: 10.0,
            maxhealth: 10.0,

            weapon:  Sword {
                name: String::from("Stick"),
                normal: 1.5,
                bonus: Bonus::Zero
            },

            armour: Armour {
                name: String::from("Wooden armour"),
                normal: 1.5,
                bonus: Bonus::Zero
            }

        },

        Character{
            name: String::from("knight"),
            health: 10.0,
            maxhealth: 10.0,

            weapon:  Sword {
                name: String::from("the hollow one"),
                normal: 1.5,
                bonus: Bonus::Zero
            },

            armour: Armour {
                name: String::from("Cloth piece"),
                normal: 1.5,
                bonus: Bonus::Zero
            }
        },

    ];
    
    // player selecting from characters
    let choosen_character:usize = async_std::task::block_on(chooseCharacter( characters.clone() ));

    let mut game: Game = Game::new(characters[choosen_character].to_owned());

    let mut stdout = stdout().into_raw_mode().unwrap();
    
    
    // where the game runs
    loop {
        
        async_std::task::block_on( get_next_step() );
    
        
         game.generate_enemy();

         game.announce_enemy(&mut stdout);

        // boss every 10 rounds
        if game.level % 10 == 0{
          game.generate_enemy();
        }

        //this blocks/halts the whole program, and everyting  inside this function is async
        let killed: bool = async_std::task::block_on( game.fight_enemy() );     
        
        if !killed {
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