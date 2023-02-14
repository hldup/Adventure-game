
use std::{thread, time, vec};


/// this need improvement cus its retarded
/// Hitrange: i16-i16, like 2-4 or 1-2. 
/// How it would look like
/// 2-4     =[=|]===========
/// 6-12    =====[=====]=|==
/// player needs to press enter when his cursor is betweem these ranges
pub fn displayHitBar(hitRange: vec![i16,i16]){
    let mut marker: i32 = 0; ///where the hitmarker is 
    let mut backwardsCounter: i32 = 0; // when its positive the marker value decreses
    let mut printString:String = String::new(); 

    /// TODO dynamic range display
    /// 
        loop{

            thread::sleep(time::Duration::from_millis(10)); // DIFFICUTLY: The harder the enemies are the less this number gets.


            printString.clear();

            if marker == 16 { backwardsCounter+= 16 } // if its at the end
            if marker == 0 { backwardsCounter = 0 } // when it arrives at pos 0 again


        for i in 0..16  {   
            if i == marker{ printString.push_str("|"); }
            else { printString.push_str("="); }
        }

        if backwardsCounter > 0 { marker-=1 }
        else{ marker+= 1 }


        println!("{}",printString);
    }
}