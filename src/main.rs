// importus
use std::{vec, io::stdout};
use game::{Character, Game, PotionInventory, Enemy};
use option_selector::chooseCharacter;
use crossterm::{
    event::{DisableMouseCapture},
    execute,
    terminal::enable_raw_mode,
};

// modules
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
            attack: 2.0,
            health: 3.0,
            protection:43.0,
        },
        Character{
            name: String::from("gecigranat"),
            attack: 2.0,
            health: 532.0,
            protection:4.0,
        },
        Character{
            name: String::from("teszkarakter"),
            attack: 2.0,
            health: 421.0,
            protection:4.0,
        },
        Character{
            name: String::from("xddd"),
            attack: 2.0,
            health: 34.0,
            protection:14.0,
        },
    ];


    // player selecting from characters
    let mut choosen_character:usize = 0;

    async_std::task::block_on(chooseCharacter(characters.clone(), choosen_character));


    // let mut stdout = stdout().into_raw_mode().unwrap();
    // start_countdown(stdout);



    let mut game: Game = Game{
        character: characters[choosen_character].to_owned(),
        round: 0,
        xp: 0.0,
        level:0,
        potions: PotionInventory::new(),
        enemy: Enemy { name: String::from(""), faction: game::Faction::Flesh, health: 0.0, damage: 0.0, xp: 0.0 }
    };
    





    // TODO after enemy is defeated generate another ane and announce the name of it & the obstacle
    loop {

        //this function takes ownership of the receiver `self`, which moves `jatke`
        game.generate_enemy();
        // announce enemy here

        //this blocks/halts the whole program, and everyting  inside this function is async
        async_std::task::block_on( game.fight_enemy() );             
            
    }



}


/*
https://piped.lunar.icu/watch?v=VAo065vRO4Q&t=102

My mother told me
Some day I wil buy (buy)
Galley with good oears
Sail to distans shores!

 */