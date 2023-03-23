use core::panic;
use std::io::stdin;
// importus
use std::{vec, io::stdout};

use Adventure_game::game::items::{Bonus, Armour,Sword};
use Adventure_game::game::{Game, Character};
use Adventure_game::option_selector::chooseCharacter;
use Adventure_game::tui::{Tui};
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
                normal: 5.5,
                bonus: Bonus::Zero
            }

        },

        Character{
            name: String::from("knight"),
            health: 5.0,
            maxhealth: 5.0,

            weapon:  Sword {
                name: String::from("the hollow one"),
                normal: 4.5,
                bonus: Bonus::Zero
            },

            armour: Armour {
                name: String::from("Cloth piece"),
                normal: 2.3,
                bonus: Bonus::Zero
            }
        },
        Character{
            name: String::from("mage"),
            health: 1.5,
            maxhealth: 1.5,

            weapon:  Sword {
                name: String::from("broken ward"),
                normal: 9.5,
                bonus: Bonus::Zero
            },

            armour: Armour {
                name: String::from("Cloth piece"),
                normal: 0.5,
                bonus: Bonus::Has { 
                    faction: Adventure_game::game::enemy::Faction::Void, 
                    amount: 4.0
                 }
            }
        },

    ];
    
    // player selecting from characters

    let mut stdout = stdout().into_raw_mode().unwrap();
    
    let reader = EventStream::new();
    let mut stdin = stdin();

    let mut terminal: Tui = Tui::new(
        &mut stdout ,
        reader,
        &mut stdin
        );

    let choosen_character:usize =  terminal.choosen_character(characters.clone());
    let mut game: Game = Game::new(characters[choosen_character].to_owned());

    // where the game runs
    loop {
        
        match terminal.get_next_step() {
            
            // fight enemy
            1 => {
                game.generate_enemy();

                // boss every 10 rounds
                if game.level % 10 == 0{
                    game.generate_enemy();
                }

                // TODO: block/async ] also quality of life: user can skip
                // TODO migrate from game to tui
                // game.announce_enemy(&mut stdout);
                
                let killed: bool = async_std::task::block_on( game.fight_enemy() );     

                if !killed {
                    game.announce_death(&mut stdout);
                    break;
                }
                
                game.enemy_killed();

            }

            // inventory
            2 => {
                async_std::task::block_on( terminal.inventory( &mut game ) );

            }
            
            // upgrade
            3 => {
                game.xp += 100.0;
                async_std::task::block_on( terminal.upgrade( &mut game ) );
            }
            4 => {}

            
            0 => { 
                break;
             }
            _=> {}
        }




        //this blocks/halts the whole program, and everyting  inside this function is async
        


    }
}


/*
https://piped.lunar.icu/watch?v=VAo065vRO4Q&t=102

My mother told me
Some day I wil buy (buy)
Galley with good oears
Sail to distans shores!

 */