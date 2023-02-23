
// everthing is i128 since the game goes indefinetly and values will rise. therefore some stats will overflow i8-i64

pub struct  Character {
    pub name: String,
    pub attack: i128,
    pub health: i128,
    pub protection: i128,
}


pub enum Potions {
    SmallHeal { amount: i128 }, // heals 30%
    MediumHeal { amount: i128 }, // heals 50%
    LargeHeal { amount: i128 }, // heals 70%
    SmallStrenght { amount: i128 }, // enpowers next attack by 15%
    LargeStrenght { amount: i128 }, // enpowers next attack by 45%
    Invisibility { amount: i128 } // gives a 1/2 chance to dodge enemies attack
}

pub struct PotionInventory{
    small_heal: i128,
    medium_heal: i128,
    large_heal: i128,
    small_strength: i128,
    large_strength: i128,
    invisibility: i128,
}

// same here, everything i128
pub struct Game {
    /// character stats like attack, health and dodge are edited before attacks
    character: Character,
    round: i128,
    xp:i128, 
    potions: PotionInventory,

}