// importus
use std::{vec, io::{stdout, Write}};
use game::{Character, Game, PotionInventory, Enemy};
use option_selector::chooseCharacter;
use tui::{hitbar};

use crossterm::{
    event::{DisableMouseCapture},
    execute,
    terminal::enable_raw_mode,
};

mod game;
mod tui;
mod option_selector;
mod input_handler;

fn main() { 


    enable_raw_mode();
    let mut stdout = stdout();
    execute!(stdout, DisableMouseCapture);


    let mut characters: Vec<Character> = vec![
        Character{
            name: String::from("Gyulameleg"),
            attack: 22,
            health: 3,
            protection:43,
        },
        Character{
            name: String::from("gecigranat"),
            attack: 2,
            health: 532,
            protection:4,
        },
        Character{
            name: String::from("teszkarakter"),
            attack: 2,
            health: 421,
            protection:4,
        },
        Character{
            name: String::from("xddd"),
            attack: 22,
            health: 34,
            protection:14,
        },
    ];


    // player selecting from characters
    let mut choosen_character:usize = 0;

    async_std::task::block_on(chooseCharacter(characters.clone(), choosen_character));


    // let mut stdout = stdout().into_raw_mode().unwrap();
    // start_countdown(stdout);


    writeln!(stdout, "generating shit game");

    let mut jatke: Game = Game{
        character: characters[choosen_character].to_owned(),
        round: 0,
        xp: 0,
        potions: PotionInventory{
            small_heal: 0,
            medium_heal: 0,
            large_heal: 0,
            small_strength: 0,
            large_strength: 0,
            invisibility: 0,
        },
        enemy: Enemy { name: String::from(""), faction: game::Faction::Flesh, health: 0, damage: 0, xp: 0 }
    };

    jatke.generate_enemy();

    async_std::task::block_on( hitbar(vec![4,6],jatke));

    // loop {
    //     let mut enemy: Enemy = generate_enemy(jatke.to_owned());

        
    // }        
        // generate random enemy 
        // generate random obstacle/ challange
        // - typing challange
        // - hitmarker
        


        //display challage in the middle of the screen with description

}