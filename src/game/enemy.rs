use super::items::{Item};




#[derive(Debug, Clone,Copy)]
pub enum Faction {
    Skeleton,
    Flesh,
    Void,
}
#[derive(Debug,Clone)]
pub struct Enemy {
    
    pub name: String, // cool ass name
    pub faction: Faction, // faction, idk maybe they will have some resistance
    pub health: f64, // health points
    pub damage:  f64, // damage it deals to player
    pub xp: f64, // how much xp does the enemy give as reward
    pub reward: Item,
}
