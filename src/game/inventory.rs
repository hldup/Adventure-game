use std::collections::HashMap;

use super::items::{Potion, Armour, Sword};

#[derive(Debug,Clone)]

pub struct Inventory{
    pub potions: HashMap<String, Potion>,
    pub armours: HashMap<String,Armour>,
    pub swords: HashMap<String, Sword>,

}
impl  Inventory {
    
    pub fn new() -> Inventory{
        Inventory { 
            potions: HashMap::new(), 
            armours: HashMap::new(), 
            swords: HashMap::new()
         }

    }
}