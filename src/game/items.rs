use super::enemy::Faction;

#[derive(Debug,Clone)]
pub struct BonusProtection{
    pub faction: Faction,
    pub amount: f64
}

#[derive(Debug,Clone)]
pub struct  Armour {
    pub protection: f64,
    pub bonus_protection: BonusProtection,
}

#[derive(Debug,Clone)]
pub struct BonusDamage {
    pub faction: Faction,
    pub amount: f64
}

#[derive(Debug,Clone)]
pub struct Sword{
    pub attack: f64,
    pub bonus_dmg: BonusDamage,
}

#[derive(Debug,Clone)]
pub enum Effect {
    Heal,
    Protection,
    Attack
}

#[derive(Debug,Clone)]
pub struct Potion {
    pub effect: Effect, // attack
    pub amount: f64, //3
    pub one_use: bool // true
}




#[derive(Debug,Clone)]
pub enum Item {
    Sword {
        data: Sword,
    },
    Armour {
        data: Armour,
    },
}