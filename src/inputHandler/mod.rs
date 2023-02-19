use rdev::{listen, Event, Key};

/* #[derive(Debug)]
pub enum  Key {
    Delete,
    Enter,
    Escape,
    Char{
        value: String, // for example: a p v x , . / ` | on the keyboard
    },
    Void, // a way to ignore static typing
} */
/* 
pub fn FilterInputStreamForKeys(event:Event) -> Key{

    // Filtering out event that are only ButtonPresses
    match event.event_type {

        ButtonPress =>{
            match event.name {
                Some(key) =>{ // filtering out Some (some -> String)


                    match key.as_str() {
                            "\r" => return Key::Enter, 
                            r"\u{8}" => return Key::Delete,
                            r"\u{1b}" => return Key::Escape,
                            &_ => return  Key::Char { value: key }, // returning void as a workaround
                        };

                }
                None => return Key::Void
            }
        }
        
        // If its not a ButtonPress do nada
        _ => todo!("asd")
    }
}
 */
#[derive(Debug)]
pub enum GameDirectionKey {
    UpArrow, 
    DownArrow, 
    LeftArrow, 
    RightArrow,
    Void,
    Enter,
}


pub fn FilterInputStreamForArrows(event:Event) -> GameDirectionKey{

    // Filtering out event that are only ButtonPresses
    match event.event_type {

        rdev::EventType::KeyPress(n) =>{

            match n {
                Key::LeftArrow =>   GameDirectionKey::LeftArrow,
                Key::KeyH =>   GameDirectionKey::LeftArrow,

                Key::RightArrow =>  GameDirectionKey::RightArrow,
                Key::KeyL =>  GameDirectionKey::RightArrow,
                

                Key::UpArrow => GameDirectionKey::UpArrow,
                Key::KeyK => GameDirectionKey::UpArrow,

                Key::DownArrow => GameDirectionKey::DownArrow,
                Key::KeyJ => GameDirectionKey::DownArrow,
                

                Key::Space => GameDirectionKey::Enter,
                 
                Key =>  GameDirectionKey::Void,
                

            }

/*             let allowed_keys: Vec<Key> = vec![
                Key::LeftArrow,  
                Key::RightArrow, 
                Key::UpArrow,
                Key::DownArrow,  
            ];

            if allowed_keys.contains(&n) {
                return n;
            }else{
                return Key::End;
            } */

        }
        
        // If its not a ButtonPress do nada
        _ => return GameDirectionKey::Void,
    }
}