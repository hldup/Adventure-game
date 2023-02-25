use std::time::Duration;

use termion::event::Key;

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


use futures::{future::FutureExt, select, StreamExt};
use futures_timer::Delay;

use crossterm::{
    cursor::position,
    event::{DisableMouseCapture, EnableMouseCapture, Event, EventStream, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};

pub async fn enter(){
    let mut reader = EventStream::new();

    loop {
        let mut delay = Delay::new(Duration::from_millis(1_000)).fuse();
        let mut event = reader.next().fuse();


        
        select! {
            _ = delay => { }, // do nothing if its delay

            maybe_event = event => {
                match maybe_event {
                    Some(Ok(event)) => {
                        println!("Event::{:?}\r", event);

                        if event == Event::Key(KeyCode::Char('c').into()) {
                            println!("Cursor position: {:?}\r", position());
                        }

                        if event == Event::Key(KeyCode::Esc.into()) {
                            break;
                        }
                    }
                    Some(Err(e)) => println!("Error: {:?}\r", e),
                    None => break,
                }
            }
        };
    }

}