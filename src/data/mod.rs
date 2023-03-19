use async_std::fs;

use crate::game::Game;

pub struct Save {
    game: Game, 
}

impl Save {
    pub fn new( game:Game ) -> Save{
        Save { game: game }
    }
    pub fn toFile( path: String ){
        // asd
    }
    
}