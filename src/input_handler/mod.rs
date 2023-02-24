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