use termion::event::Key;

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

    
    

pub fn FilterInputStreamForArrows(key:Key) -> GameDirectionKey{

    // Filtering out event that are only ButtonPresses
    match key {

        Key::Left =>   GameDirectionKey::LeftArrow,
        Key::Char('h') =>   GameDirectionKey::LeftArrow,

        Key::Right =>  GameDirectionKey::RightArrow,
        Key::Char('l') =>  GameDirectionKey::RightArrow,
        

        Key::Up => GameDirectionKey::UpArrow,
        Key::Char('k') => GameDirectionKey::UpArrow,

        Key::Down => GameDirectionKey::DownArrow,
        Key::Char('j') => GameDirectionKey::DownArrow,
        

        Key::Char('\n') => GameDirectionKey::Enter,

        _ => GameDirectionKey::Void,

    }
}