use core::panic;
use std::io::stdin;
// importus
use std::{vec, io::stdout};

use Adventure_game::game::items::{Bonus, Armour,Sword};
use Adventure_game::game::{Game, Character};
use Adventure_game::option_selector::chooseCharacter;
use Adventure_game::tui::{get_next_step, Tui};
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

    ];
    
    // player selecting from characters
    let choosen_character:usize = async_std::task::block_on(chooseCharacter( characters.clone() ));

    let mut game: Game = Game::new(characters[choosen_character].to_owned());

    let mut stdout = stdout().into_raw_mode().unwrap();
    
    let reader = EventStream::new();
    let mut stdin = stdin();

    let mut terminal: Tui = Tui::new(
         &mut stdout ,
         reader,
        &mut game,
        &mut stdin
        );
    

    
    async_std::task::block_on( terminal.show_inventory() );
    return;

    // where the game runs
    loop {
        
        match get_next_step(&mut stdout) {
            
            // fight enemy
            1 => {
                game.generate_enemy();

                // boss every 10 rounds
                if game.level % 10 == 0{
                    game.generate_enemy();
                }

                // TODO: block/async ] also quality of life: user can skip
                game.announce_enemy(&mut stdout);
                
                let killed: bool = async_std::task::block_on( game.fight_enemy() );     

                if !killed {
                    game.announce_death(&mut stdout);
                    break;
                }
                
                game.enemy_killed();

            }

            // inventory
            2 => {

            }
            
            3 => {}
            4 => {}

            
            0 => { panic!("quit") }
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