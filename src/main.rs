// importus
use std::{vec, io::stdout, io::Write};

use Adventure_game::game::items::Bonus;
use Adventure_game::game::{Character, items::Item};
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
            name: String::from("Ziak"),
            health: 3.0,

            weapon:  Item { 
                tipus: Adventure_game::game::items::ItemType::Sword,
                name: String::from("The lost one"),
                normal: 2.0,
                bonus: Bonus::Zero,
             },
            armour: Item { 
                tipus: Adventure_game::game::items::ItemType::Armour,
                name: String::from("Chectpiece"),
                normal: 2.0,
                bonus: Bonus::Zero,
             },
        },
        Character{
            name: String::from("Skinwalker"),
            health: 3.0,

            weapon:  Item { 
                tipus: Adventure_game::game::items::ItemType::Sword,
                name: String::from("weed wacker"),
                normal: 2.0,
                bonus: Bonus::Zero,
             },
            armour: Item { 
                tipus: Adventure_game::game::items::ItemType::Armour,
                name: String::from("Boots"),
                normal: 2.0,
                bonus: Bonus::Zero,
             },
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
        let outcome = async_std::task::block_on( game.fight_enemy() );     
        
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