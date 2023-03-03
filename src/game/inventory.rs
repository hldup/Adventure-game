use std::collections::HashMap;

use super::items::Item;


#[derive(Debug,Clone)]

pub struct Inventory{
    pub items: HashMap<String, Item>,
}
impl  Inventory {
    
    pub fn new() -> Inventory{
        Inventory{
            items: HashMap::new()
        }

    }
}