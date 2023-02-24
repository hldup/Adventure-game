// importus
use std::vec;
use game::{Character, Game, PotionInventory, Enemy, generate_enemy};
use option_selector::chooseCharacter;
use tui::hitbar;
mod game;
mod tui;
mod option_selector;
mod input_handler;


fn main(){

    
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
    let choosen_character:usize = chooseCharacter(characters.clone());

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
        }
    };


    loop{
   
        hitbar(vec![1,4]);

        // generate random enemy 
        // generate random obstacle/ challange
        // - typing challange
        // - hitmarker
        
        //display challage in the middle of the screen with description


        
    };


}