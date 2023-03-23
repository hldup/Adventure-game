use super::items::{Sword, Armour, Potion};
use serde::{Serialize, Deserialize};




#[derive(Debug, Clone,Copy,PartialEq,Serialize, Deserialize)]
pub enum Faction {
    Skeleton,
    Flesh,
    Void,
    Zero,
}
#[derive(Debug, Clone,PartialEq,Serialize,Deserialize)]
pub enum RewardType{
    Sword{
        data: Sword,
    },

    Armour{
        data: Armour
    },

    Potion{
        data: Potion,
    }
}
#[derive(Debug, Clone,PartialEq,Serialize,Deserialize)]
pub enum Reward{
    None,
    Item{
        tipus: RewardType
    }
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Enemy {
    
    pub name: String, // cool ass name
    pub faction: Faction, // faction, idk maybe they will have some resistance
    pub health: f64, // health points
    pub damage:  f64, // damage it deals to player
    pub xp: f64, // how much xp does the enemy give as reward
    pub reward: Reward, // sword armour potion

}
