
// everthing is i128 since the game goes indefinetly and values will rise. therefore some stats will overflow i8-i64

use std::vec;

use rand::Rng; // 0.8.5


#[derive(Debug, Clone)]
pub struct  Character {
    pub name: String,
    pub attack: i128,
    pub health: i128,
    pub protection: i128,
}


// pub enum Potions {
//     SmallHeal { amount: i128 }, // heals 30%
//     MediumHeal { amount: i128 }, // heals 50%
//     LargeHeal { amount: i128 }, // heals 70%
//     SmallStrenght { amount: i128 }, // enpowers next attack by 15%
//     LargeStrenght { amount: i128 }, // enpowers next attack by 45%
//     Invisibility { amount: i128 } // gives a 1/2 chance to dodge enemies attack
// }

#[derive(Debug,Clone)]
pub struct PotionInventory{
    pub small_heal: i128,
    pub medium_heal: i128,
    pub large_heal: i128,
    pub small_strength: i128,
    pub large_strength: i128,
    pub invisibility: i128,
}
#[derive(Debug, Clone)]
pub enum Faction {
    Skeleton,
    Flesh,
    Void,
}
#[derive(Debug,Clone)]
pub struct Enemy {
    pub name: String,
    pub faction: Faction,
    pub health: i128,
    pub damage:  i128,
    pub xp: i128,
}
// same here, everything i128
#[derive(Debug,Clone)]
pub struct Game {
    /// character stats like attack, health and dodge are edited before attacks
    pub character: Character,
    pub round: i128,
    pub xp:i128, 
    pub potions: PotionInventory,
    pub enemy: Enemy,
}


impl Game {
    pub fn generate_enemy(&mut self){

        let factions: Vec<Faction> = vec![ 
            Faction::Skeleton,
            Faction::Flesh,
            Faction::Void
            ];
        
        self.enemy = Enemy { 
        
            // TODO cool name generator like WariZkorzok or idk...
            name: String::from("asd"),
            faction: factions[rand::thread_rng().gen_range(0..factions.len())].to_owned(),
        
            health: self.character.health + ((self.character.health as f64 * 0.15) as i128),

            damage: rand::thread_rng().gen_range(
                self.character.attack.. 
                   // converting i128 to float, multiplying by float then turning it back to i128
                  (self.character.attack as f64 + self.character.attack as f64 * 0.15) as i128
                ),
        
            // 5u get 10% xp from enemy based on ur xp
            // so for 100 xp u get 10
            xp: (self.xp as f64 * 0.15) as i128, 
    };

}
}



