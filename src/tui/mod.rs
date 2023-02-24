
use std::{thread, time::{self, Duration}, vec, io::{stdin, stdout,Write}};
use async_std::stream::StreamExt;
use crossterm::{
    cursor::position,
    event::{DisableMouseCapture, EnableMouseCapture, Event, EventStream, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};

use futures::{FutureExt, select};
use futures_timer::Delay;
use termion::{raw::{IntoRawMode, RawTerminal}, color};

use crate::game::Game;


/// this need improvement cus its retarded
/// Hitrange: i16-i16, like 2-4 or 1-2. 
/// How it would look like
/// 2-4     =[=|]===========
/// 6-12    =====[=====]=|==
/// player needs to press enter when his cursor is betweem these ranges

pub async fn hitbar(hit_range: Vec<i32> ,game:Game) {
    
    let mut marker: i32 = 0; // where the hitmarker is 
    let mut backwards_counter: i32 = 0; // when its positive the marker value decreses
    let mut print_string:String = String::new(); 
    let mut reader = EventStream::new();


    // console 
    let mut stdout = stdout().into_raw_mode().unwrap();
    let (x, y) = termion::terminal_size().unwrap();
    writeln!( stdout, "{}",  termion::clear::All).unwrap();
   

        loop{

            let mut delay = Delay::new(Duration::from_millis(500)).fuse();
            writeln!( stdout, "{}",  color::Fg(color::Reset)).unwrap();
            let mut event = reader.next().fuse();


            writeln!( stdout, 
                "{} {} {} {} {} {}", 

                termion::cursor::Goto(1,y-4),
                format!("Health: {} {} {}", color::Bg(color::Red), game.character.health, color::Bg(color::Reset)),                
                
                termion::cursor::Goto(1,y-3),
                format!("Attack: {} {} {}", color::Bg(color::LightYellow), game.character.attack, color::Bg(color::Reset)),                


                termion::cursor::Goto(1,y-2),
                format!("Protection: {} {} {}", color::Bg(color::LightCyan), game.character.protection, color::Bg(color::Reset)),                

                ).unwrap();


            print_string.clear();
            if marker == 16 { backwards_counter+= 16 } // if its at the end
            if marker == 0 { backwards_counter = 0 } // when it arrives at pos 0 again

            for i in 0..16  {   
    
                if i == marker { print_string.push_str("|"); }
    
                else { 
                    if i == hit_range[0]{
                    print_string.push_str("[");   
                    }
                    else{
                        print_string.push_str("=");
                    } 
    
                    if i == hit_range[1]{
                        print_string.push_str("]");
                    }else{
                        print_string.push_str("=");
                    }
                 }
            
            }
    
            if backwards_counter > 0 { marker-=1 }
            else{ marker+= 1 }
    
            writeln!( stdout, 
                "{} {}", 
                termion::cursor::Goto(( (x as usize -print_string.len() ) / 2) as u16 , y/2),
                print_string,
                ).unwrap();


            // filtering input for enter  & space and adjusting scores according to it
            select! {
                _ = delay => {  },
                maybe_event = event => {
                    match maybe_event {
                        Some(Ok(event)) => {
                                if event == Event::Key( KeyCode::Enter.into() )  {
                                    

                                    // if player hit between hitrange
                                    if  hit_range[0] <= marker && marker <= hit_range[1]{
                                        writeln!( stdout, 
                                            "{} {} Hit!", 
                                            termion::cursor::Goto(3,5),
                                            color::Fg(color::Green),
                                            ).unwrap();    
                                    
                                    }else{
                                    // if not, increase speed and take one from the dmg chances
                                    writeln!( stdout, 
                                        "{} {}Miss!", 
                                        termion::cursor::Goto(3,5),
                                        color::Fg(color::Red),
                                        ).unwrap();    
                                    }
                }                // exit imnplementation
                                if event == Event::Key(KeyCode::Esc.into()) || event == Event::Key(KeyCode::Char('q').into()) { break; }        
                            } // end of match ok scene

                        Some(Err(e)) => println!("Error: {:?}\r", e),
                        None => break,
                    }
                }
            };
;


    }

}



pub fn start_countdown(mut stdout: RawTerminal<std::io::Stdout>){
    
    let (x, y) = termion::terminal_size().unwrap();

    let mut i = 3;
   while i > 0 {
      writeln!( stdout, 
        "{} {} {}",
        termion::clear::All, 
        termion::cursor::Goto(x / 2, y / 2),
        i,
        ).unwrap();
        i-=1;
        thread::sleep(time::Duration::from_millis(1000)); 
  }


}

