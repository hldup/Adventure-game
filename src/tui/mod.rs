
use std::{thread, time, vec, io::{stdin, stdout,Write}};
use termion::raw::IntoRawMode;


/// this need improvement cus its retarded
/// Hitrange: i16-i16, like 2-4 or 1-2. 
/// How it would look like
/// 2-4     =[=|]===========
/// 6-12    =====[=====]=|==
/// player needs to press enter when his cursor is betweem these ranges

pub fn hitbar(hit_range: Vec<i32> ){
    let mut marker: i32 = 0; // where the hitmarker is 
    let mut backwards_counter: i32 = 0; // when its positive the marker value decreses
    let mut print_string:String = String::new(); 


    // console 
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let (x, y) = termion::terminal_size().unwrap();
    let termWidth = usize::try_from(x).expect("failed to covnert");

    // TODO dynamic range display
        loop{
            thread::sleep(time::Duration::from_millis(100)); // DIFFICUTLY: The harder the enemies are the less this number gets.
            
            writeln!( stdout, "{}",  termion::clear::All).unwrap();
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
            termion::cursor::Goto(( (x as usize -print_string.len() ) / 2) as u16 ,1),
            print_string,
            ).unwrap();
    }
}

