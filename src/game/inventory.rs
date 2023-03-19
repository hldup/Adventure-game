use std::collections::HashMap;

use rand::Rng;

use super::{items::{Potion, Armour, Sword}, enemy::{RewardType, Reward}};

#[derive(Debug,Clone)]

pub struct Inventory{
    pub potions: HashMap<i128, Potion>,
    pub armours: HashMap<i128,Armour>,
    pub swords: HashMap<i128, Sword>,
}

impl  Inventory {
    
    pub fn new() -> Inventory{
        Inventory { 
            potions: HashMap::new(), 
            armours: HashMap::new(), 
            swords: HashMap::new()
         }
    }

    pub fn add( &mut self, reward: Reward ){

        let mut key: i128 = rand::thread_rng().gen_range(0..9999999999);

        match reward {
             
            Reward::None => {},

            Reward::Item { tipus: reward } => {

               match reward {
                   
                   RewardType::Armour{ data: n} =>{

                    while self.armours.contains_key(&key){
                        key=rand::thread_rng().gen_range(0..9999999999);
                    }
                    self.armours.insert(key, n,  );
                   

                   }

                   RewardType::Potion{ data: n} =>{

                    while self.potions.contains_key(&key){
                        key=rand::thread_rng().gen_range(0..9999999999);
                    }
                    self.potions.insert(key, n,  );
                   }

                   RewardType::Sword{ data: n} =>{
                    
                    while self.swords.contains_key(&key){
                        key=rand::thread_rng().gen_range(0..9999999999);
                    }
                    self.swords.insert(key, n,  );
                   
                   }
                   
               }
            }

        }


    }

}