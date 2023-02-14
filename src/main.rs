use std::process::exit;
use rdev::{listen, Event};
use tui::displayHitBar;
mod tui;


#[derive(Debug)]
enum  Key {
    Delete,
    Enter,
    Escape,
    Char{
        value: String, // for example: a p v x , . / ` | on the keyboard
    },
    Void, // a way to ignore static typing
}

fn FilterInputStreamForKeys(event:Event) -> Key{

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


fn main(){


let mut username: String = String::new();

// listener 
listen(
    // moving shit
move |event:Event | -> () {
    let key:Key = FilterInputStreamForKeys(event);
    // Key::Char{ value: String }   
    match key {
        Key::Char { value: n } =>{
            username.push_str(n.as_str())
        },

        Key::Delete => {
            username.remove(username.len() +1 ); 
            return ()
        },

        Key::Escape => {
            exit(0)
        },
        // ew 
        Key::Void =>{
            return ()
        },

        Key::Enter =>{
          // go to next stage

            displayHitBar();

            return ()
        }

    }

}).expect("asd");








}