use std::process::exit;
use optionSelector::chooseCharacter;
use rdev::{listen, Event};
use tui::{displayHitBar};
mod tui;
mod optionSelector;
mod inputHandler;

pub struct  Character {
    name: String,
    attack: i16,
    health: i16,
    protection: i16,
}



fn main(){


let u = chooseCharacter(vec![
    Character{
        name: String::from("Gyulameleg"),
        attack: 2,
        health: 3,
        protection:4,
    },
    Character{
        name: String::from("gecigranat"),
        attack: 2,
        health: 3,
        protection:4,
    },
    Character{
        name: String::from("teszkarakter"),
        attack: 2,
        health: 3,
        protection:4,
    },
    Character{
        name: String::from("xddd"),
        attack: 2,
        health: 3,
        protection:4,
    },

]);


/* optionSelector::chooser(
    vec![
        String::from("option 1"),
        String::from("option 2")
    ],
    String::from("Choose a character")
);
 */






}