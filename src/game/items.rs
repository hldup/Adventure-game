use super::enemy::Faction;

#[derive(Debug,Clone)]
pub enum Bonus {
    Zero,
    Has{
        faction: Faction,
        amount: f64
    }
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
pub enum ItemType {
    Sword,
    Armour,
    Zero
}



#[derive(Debug,Clone)]
pub struct Item {
    pub tipus: ItemType,
    pub name: String,
    pub normal: f64,
    pub bonus: Bonus,
}