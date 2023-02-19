
use rdev::{listen, Event};

use crate::{inputHandler::{FilterInputStreamForArrows, GameDirectionKey}, Character, tui::centerTextForConsole};


pub fn chooser(
    options: Vec<String>,
    display_text: String
    ) -> i8{

    let (x, y) = termion::terminal_size().unwrap();
    let termWidth = usize::try_from(x).expect("failed to covnert");

    let mut selectionText: String = String::new();
    // putting text in the middle
    for i in 0..termWidth {
        if ( termWidth - display_text.len() ) / 2 <= i {
        selectionText.push_str(display_text.as_str());
        break
        } 
            selectionText.push_str(" ")
     }


    println!("{}",selectionText);

    return 3
}   



pub fn chooseCharacter(characters: Vec<Character>) -> i8 {

let (x, y) = termion::terminal_size().unwrap();
let termWidth = usize::try_from(x).expect("failed to covnert");

let mut print: String = String::new();
let mut index: usize = 0;

let mut returnValue = -1;

//pls ignore this
print.push_str(centerTextForConsole(termWidth, format!("{} \n \n", characters[0].name.clone() ) ).as_str()  );
print.push_str(centerTextForConsole(termWidth, format!("attack: {} \n", characters[0].attack.clone() ) ).as_str() );
print.push_str(centerTextForConsole(termWidth, format!("damage: {}\n", characters[0].health.clone() ) ).as_str() );
print.push_str(centerTextForConsole(termWidth, format!("defense: {}\n", characters[0].protection.clone() ) ).as_str());



print!("{}",print);
print.clear();

// listener 
listen(
    // moving shit
move |event:Event | -> () {
    let key:GameDirectionKey = FilterInputStreamForArrows(event);


    // Key::Char{ value: String }   
    match key {
        // do nothing
        GameDirectionKey::DownArrow => { }
        GameDirectionKey::UpArrow => { }



            
        // selection 
        GameDirectionKey::LeftArrow => {
            if index - 1  > 0  {
                index -= 1;
                print!("{}[2J", 27 as char); //clearing console 
                print.push_str(centerTextForConsole(termWidth, format!("{} \n \n", characters[index].name.clone() ) ).as_str()  );
                print.push_str(centerTextForConsole(termWidth, format!("attack: {} \n", characters[index].attack.clone() ) ).as_str() );
                print.push_str(centerTextForConsole(termWidth, format!("damage: {}\n", characters[index].health.clone() ) ).as_str() );
                print.push_str(centerTextForConsole(termWidth, format!("defense: {}\n", characters[index].protection.clone() ) ).as_str());
                

                print!("{}[2J", 27 as char); //clearing console 
                print!("{}",print);
                print.clear();

            }else {
                // do nothing
            }

        }
        GameDirectionKey::RightArrow => {
            if index +1 < characters.len()  {
                index += 1;

                print!("{}[2J", 27 as char); //clearing console 
                
                print.push_str(centerTextForConsole(termWidth, format!("{} \n \n", characters[index].name.clone() ) ).as_str()  );
                print.push_str(centerTextForConsole(termWidth, format!("attack: {} \n", characters[index].attack.clone() ) ).as_str() );
                print.push_str(centerTextForConsole(termWidth, format!("damage: {}\n", characters[index].health.clone() ) ).as_str() );
                print.push_str(centerTextForConsole(termWidth, format!("defense: {}\n", characters[index].protection.clone() ) ).as_str() );
                
                print!("{}[2J", 27 as char); //clearing console 
                print!("{}",print); // pritning the charachter data
                print.clear();// clearing the temp string

            }else {
                // do nothing
            }
        }
        
            GameDirectionKey::Enter => {
                returnValue = i8::try_from(index).expect("out of range");
            }

        GameDirectionKey::Void => {

        }   


    }

}).expect("asd");



return  returnValue;


}