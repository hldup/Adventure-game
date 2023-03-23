use std::{path::Path, fs::{File, self}, io::Write};


use crate::game::Game;

pub struct Save {
    pub path: String,
}
impl Save {

    pub fn to_file( path: String, game: Game ){
    
       if !Path::new( &path ).exists() { 
        File::create(&path).expect("Failed to create");
       
       }
       let mut file = File::open(path).expect("Failed to open file");
       let data = serde_json::to_string( &game ).expect("Failed to seralize game DATA");
       file.write_all( data.as_bytes() ).expect("Failed to write to file");
        
    
    }

     pub fn from_file( path: String ) -> Game{
        let game: Game = serde_json::from_str( 
            fs::read_to_string(path).expect("Failed to read to string").as_str() 
        ).expect("failed to generate game");
         

        return  game;
     }
    
}