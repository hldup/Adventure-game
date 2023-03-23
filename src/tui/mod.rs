
use std::{thread, time::{self, Duration}, vec, io::{stdin, stdout,Write, Stdout, SeekFrom, Stdin}, collections::HashMap, ops::Add, fmt::format};
use async_std::stream::StreamExt;

use crossterm::{
    event::{ Event, EventStream, KeyCode}, cursor,
};

use futures::{FutureExt, select};
use futures_timer::Delay;
use rand::{thread_rng, Rng, distributions::Alphanumeric};
use termion::{raw::{ RawTerminal}, color::{self, Reset}, input::TermRead};

use crate::{game::{Game, items::{Potion, Sword, Armour}, Character}, input_handler::{FilterInputStreamForArrows, GameDirectionKey}};


// TOP BAR
fn display_level(stdout:&mut RawTerminal<Stdout>, game:Game, x:u16, y:u16){
    writeln!( stdout, 
        "{} {}", 

        termion::cursor::Goto(x/2,3),
        format!("Level {}", game.level),
        ).unwrap();

}


// LEFT BAR
fn display_stats(stdout:&mut RawTerminal<Stdout>, game:Game, x:u16, y:u16){


    writeln!( stdout, 
        "{} {} {} {} {} {} {} {}", 

        termion::cursor::Goto(1,y-4),
        format!("{} {:.2} {} Health", color::Bg(color::Red), game.character.health.round(), color::Bg(color::Reset)),                
        
        termion::cursor::Goto(1,y-3),
        format!("{} {:.2} {} Attack", color::Bg(color::LightYellow), game.character.weapon.normal.round() + game.attack.round(), color::Bg(color::Reset)),                


        termion::cursor::Goto(1,y-2),
        format!("{} {:.2} {} Protection", color::Bg(color::LightCyan), game.character.armour.normal.round(), color::Bg(color::Reset)),                


        termion::cursor::Goto(1,y-1),
        format!("{} {:.2} {} XP", color::Bg(color::LightGreen), game.xp, color::Bg(color::Reset)),                


        ).unwrap();

}

// RIGH BAR
fn display_enemy(stdout:&mut RawTerminal<Stdout>, game:Game, x:u16, y:u16){


    let enemy_faction: String = format!("Faction {} {:?} {} ", color::Bg(color::LightCyan), game.enemy.faction, color::Bg(color::Reset));
    let enemy_damage: String = format!("Attack {} {:.2} {} ", color::Bg(color::LightYellow), game.enemy.damage.round(), color::Bg(color::Reset));  
    let enemy_health: String = format!("Health {} {:.2} {}", color::Bg(color::Red), game.enemy.health.round(), color::Bg(color::Reset));                

    writeln!( stdout, 
        "{} {} {} {} {} {}", 

        termion::cursor::Goto(x-13,y-4),
        enemy_health,

        termion::cursor::Goto(x-13 as u16,y-3),
        enemy_damage,

        termion::cursor::Goto(x-16,y-2),
        enemy_faction,

        ).unwrap();

}


pub struct  Hitbar{
    pub reader: EventStream,
    pub stdout:  RawTerminal<Stdout>,
    pub game:  Game,
    
    //private
    print_string: String,
    marker: i32,
    backwards_counter: i32,
    hit_range: Vec<i32>,
    speed: u64,
}


impl Hitbar {
    // i did this just to soly practice "production" coding cus i saw this being used
    // also now i dont have to define private fields 
    pub fn new(
        reader: EventStream,
        stdout: RawTerminal<Stdout>,
        game: Game,
    ) -> Hitbar {
        Hitbar { 
            reader: reader, 
            stdout: stdout, 
            game: game, 
            print_string: String::new(),
            marker: 0,
            backwards_counter: 0,
            hit_range: vec![3,6],
            speed: 80,
         }
    }


     pub async fn play(&mut self ) -> bool{
        writeln!( self.stdout, "{}",  termion::clear::All, ).unwrap();
        loop  {

            // player go killed
            if self.game.character.health <= 0.0{
                return false
            }

            // enemy killed
            if self.game.enemy.health <= 0.0 { 
                return true;
            };


            let (x, y) = termion::terminal_size().unwrap();

             // displaying shit in the ui
             display_stats(&mut self.stdout, self.game.clone(), x,y);
             display_enemy(&mut self.stdout, self.game.clone(), x, y);
             display_level(&mut self.stdout, self.game.clone(), x, y);

            // setting color back and clearing just in case 
             writeln!( self.stdout, "{}",  color::Fg(color::Reset), ).unwrap();


             let mut delay = Delay::new(Duration::from_millis( self.speed )).fuse();
             let mut event = self.reader.next().fuse();

             self.print_string.clear();

            if self.marker == 16 { self.backwards_counter+= 16 } // if its at the end
            if self.marker == 0 { self.backwards_counter = 0 } // when it arrives at pos 0 again


            for i in 0..16  {   
    
                if i == self.marker { self.print_string.push_str("|"); }
    
                else { 
                    if i == self.hit_range[0]{
                        self.print_string.push_str("[");   
                    }
                    else{
                        self.print_string.push_str("=");
                    } 
                    if i == self.hit_range[1]{
                        self.print_string.push_str("]");
                    }else{
                        self.print_string.push_str("=");
                    }
                 }
            
            }
    
            if self.backwards_counter > 0 { self.marker-=1 }
            else{ self.marker+= 1 }
    
            writeln!( self.stdout, 
                "{} {}", 
                termion::cursor::Goto(( (x as usize - self.print_string.len() ) / 2) as u16 , y/2),
                self.print_string,
                ).unwrap();
            


            // filtering input for enter  & space and adjusting scores according to it
            select! {
                _ = delay => {  },
                maybe_event = event => {
                    match maybe_event {
                        Some(Ok(event)) => {
                                if event == Event::Key( KeyCode::Enter.into() )  {
                                

                                    // if player hit between hitrange
                                    if  self.hit_range[0] <= self.marker && self.marker <= self.hit_range[1]{
                                        writeln!( self.stdout, 
                                            "{}{}{}{} Hit!{}", 
                                            termion::cursor::Goto(x/2,y/2-1),
                                            termion::clear::AfterCursor,
                                            termion::cursor::Goto(x/2-3,y/2-1),
                                            color::Fg(color::Green),
                                            color::Fg(color::Reset),
                                            ).unwrap();

                                            self.game.hit_attack();
                                            
                                    }else{
                                    // if not, increase speed and take one from the dmg chances
                                    writeln!( self.stdout, 
                                        "{}{}{}{} Miss!{}", 
                                        termion::cursor::Goto(x/2,y/2-1),
                                        termion::clear::AfterCursor,
                                        termion::cursor::Goto(x/2-5,y/2-1),
                                        color::Fg(color::Red),
                                        color::Fg(color::Reset),
                                        ).unwrap();

                                        self.game.missed_attack();
                                    }
                                   } // exit imnplementation
                                if event == Event::Key(KeyCode::Esc.into()) || event == Event::Key(KeyCode::Char('q').into()) { panic!("exited game"); }        
                            } // end of match ok scene
                        Some(Err(e)) => println!("Error: {:?}\r", e),
                        None => return false,
                    }
                }
            };
        }
}
}



    
/*
https://piped.video/watch?v=cojoYPRcIJA&t=127

Parlons peu, pardon maman, pardon Dieu (uh-uh)
Ton gars n'est pas dangereux, il a pas b'soin d'fer, ni d'baveux (coupe, coupe)
La jalousie, ça rend miséreux, ça dévie les hommes pieux (Glock)

wish i understood french
 */




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



pub struct Tui<'a> {
    stdout: &'a mut RawTerminal<Stdout>,
    reader: EventStream,
    stdin:  &'a mut Stdin,

    // terminal cols
    x: u16,
    y:u16
}

impl<'a> Tui<'a> {

    pub fn new(
        stdout: &'a mut RawTerminal<Stdout>,
        reader: EventStream,
        stdin:  &'a mut Stdin,
        ) -> Tui<'a>{
            let (x, y) = termion::terminal_size().unwrap();
            Tui { 
                stdout:  stdout,
                reader: reader,
                stdin: stdin,
                x: x,
                y: y,
             }
    }
    pub fn get_next_step(&mut self ) -> i8 {

    
        let stdin = stdin();
        let (x, y) = termion::terminal_size().unwrap();
    
        writeln!(
            self.stdout,
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
    
            termion::clear::All,
    
            termion::cursor::Goto((x-7)/2,y/2-5),
            termion::color::Fg(color::Black),
            termion::color::Bg(color::LightWhite),
            String::from("ENTER"),
            termion::color::Fg(Reset),
            termion::color::Bg(Reset),
            String::from(" Next stage"),
    
            termion::cursor::Goto((x-7)/2,y/2-4),
            termion::color::Fg(color::Black),
            termion::color::Bg(color::LightWhite),
            String::from("I"),
    
            termion::color::Fg(Reset),
            termion::color::Bg(Reset),
            String::from(" Inventory"),
    
    
            termion::cursor::Goto((x-7)/2,y/2-3),
            termion::color::Fg(color::Black),
            termion::color::Bg(color::LightWhite),
            String::from("U"),
    
            termion::color::Fg(Reset),
            termion::color::Bg(Reset),
            String::from(" Upgrade"),
    
            termion::cursor::Goto((x-7)/2,y/2-2),
            termion::color::Fg(color::Black),
            termion::color::Bg(color::LightWhite),
            String::from("S"),
    
            termion::color::Fg(Reset),
            termion::color::Bg(Reset),
            String::from(" Stats"),
            
            termion::cursor::Goto((x-7)/2,y/2-1),
            termion::color::Fg(color::Black),
            termion::color::Bg(color::LightWhite),
            String::from("Q"),
    
            termion::color::Fg(Reset),
            termion::color::Bg(Reset),
            String::from(" Quit"),
    
    
        ).unwrap();
    
    
        for key_press in stdin.keys() {
    
            // this match case is ugly af but ig this is how rust works
            match key_press {
    
                Ok(_key) => {
                    match _key {
                        termion::event::Key::Char( _character ) => {
                            match _character {
                            
                                    // space \ next stage 
                                    '\n' => {
                                        return 1
                                    }
    
                                    //  inventory
                                    'i' =>{
                                        return 2
                                    }
                                    // upgrade
                                    'u' =>{
                                        return 3
                                    }
                                    // stats
                                    's' => {
                                        return 4
                                    }
                                    // quit
                                    'q' =>{   
                                        return 0
                                    }
    
                                _=> {}
                            }
                        }
                        
                        _=> {}
                    }
    
                }
    
    
                Err(_error) =>{}
            }
    
        }
    
        return 1;
    }
    


 // the inventory is dynamic based on the users console
 // made up of x by y rows
 // some spacing between the items and on the right it shows its stats
 // 3 tabs SWORDS ARMOUR POTIONS
 // user can EQUIP DEQUIP DELETE 
pub async fn  inventory( &mut self,  game: &mut Game){

    let mut fake_swords: HashMap<i128, Sword> = HashMap::new();

    for i in 0..20 {
        fake_swords.insert(
            rand::thread_rng().gen_range(0..9999999999),
            Sword { 
                name: rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(7)
                .map(char::from)
                .collect(),
                normal: 1.0, bonus: crate::game::items::Bonus::Zero } 
        );

    }

    let mut gui: GuiInventory = GuiInventory::new(game.inventory.potions.clone(), fake_swords, game.inventory.armours.clone(), self.stdout, self.x, self.y);

    gui.render();

    for key_press in self.stdin.keys() {

        // this match case is ugly af but ig this is how rust works
        match key_press {

            Ok(_key) => {
                match _key {

                    termion::event::Key::Left => {  
                        if gui.index -1 != 0 {
                            gui.set_index(gui.index-1);                     
                        };
                        gui.render();

                    }

                    termion::event::Key::Right => {
                        if gui.index as i128 +1  != gui.rows_x   {
                            gui.index += 1                
                        }

                        gui.render();

                    }
                    termion::event::Key::Up => {

                        if gui.rows_y - 1 != 0 {
                            gui.rows_y -= 1;
                        };
                        gui.render();

                    }

                    termion::event::Key::Down => {
                        
                        if gui.rows_y + 1 < gui.rows_y {
                            gui.rows_y += 1;
                        };

                        gui.render();

                    }
                    
                    // exiting
                    termion::event::Key::Backspace => { break}
                    termion::event::Key::Esc => { break}
                    termion::event::Key::Char('q') => { break}


                    _=> {}
                }

            }


            Err(_error) =>{}
        }
    


     }  

}

pub fn choosen_character(&mut self ,chars: Vec<Character>) -> usize {

    let mut index:usize = 0;


    writeln!( self.stdout, 
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", 
        termion::clear::All,

        termion::cursor::Goto(self.x/2-10,self.y/2),
        termion::color::Fg(color::LightBlack),
        termion::color::Bg(color::White),
        chars[index].name,
        termion::color::Bg(color::Reset),
        termion::color::Fg(color::Reset),

        termion::cursor::Goto(self.x/2-10,self.y/2-3),
        termion::color::Fg(color::LightBlack),
        termion::color::Bg(color::White),
        String::from("Health"),
        termion::color::Bg(color::Reset),
        termion::color::Fg(color::Reset),
        format!(" {}",chars[index].health),


        termion::cursor::Goto(self.x/2-10,self.y/2-2),
        termion::color::Fg(color::LightBlack),
        termion::color::Bg(color::White),
        String::from("Weapon"),
        termion::color::Bg(color::Reset),
        termion::color::Fg(color::Reset),
        format!(" {:?}",chars[index].weapon),

        termion::cursor::Goto(self.x/2-10,self.y/2-1),
        termion::color::Fg(color::LightBlack),
        termion::color::Bg(color::White),
        String::from("Armour"),
        termion::color::Bg(color::Reset),
        termion::color::Fg(color::Reset),
        format!(" {:?}",chars[index].armour),

        //  TODO complete the listing
        // name: asd
        // health
        // weapon:
        // Attack: 3, Bonus: Void +3 dmg
        // armor
        // protection: 4 Bonus:none,
        //
        
        ).unwrap();

    for key_press in self.stdin.keys() {

        // this match case is ugly af but ig this is how rust works
        match key_press {
            Ok(_key) => {
                match _key {

                    termion::event::Key::Left => {
                        if index != 0 { 
                            index -= 1;

                        writeln!( self.stdout, 
                            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", 
                            termion::clear::All,
                    
                            termion::cursor::Goto(self.x/2-10,self.y/2),
                            termion::color::Fg(color::LightBlack),
                            termion::color::Bg(color::White),
                            chars[index].name,
                            termion::color::Bg(color::Reset),
                            termion::color::Fg(color::Reset),
                    
                            termion::cursor::Goto(self.x/2-10,self.y/2-3),
                            termion::color::Fg(color::LightBlack),
                            termion::color::Bg(color::White),
                            String::from("Health"),
                            termion::color::Bg(color::Reset),
                            termion::color::Fg(color::Reset),
                            format!(" {}",chars[index].health),
                    
                    
                            termion::cursor::Goto(self.x/2-10,self.y/2-2),
                            termion::color::Fg(color::LightBlack),
                            termion::color::Bg(color::White),
                            String::from("Weapon"),
                            termion::color::Bg(color::Reset),
                            termion::color::Fg(color::Reset),
                            format!(" {:?}",chars[index].weapon),
                    
                            termion::cursor::Goto(self.x/2-10,self.y/2-1),
                            termion::color::Fg(color::LightBlack),
                            termion::color::Bg(color::White),
                            String::from("Armour"),
                            termion::color::Bg(color::Reset),
                            termion::color::Fg(color::Reset),
                            format!(" {:?}",chars[index].armour),
                    
                            //  TODO complete the listing
                            // name: asd
                            // health
                            // weapon:
                            // Attack: 3, Bonus: Void +3 dmg
                            // armor
                            // protection: 4 Bonus:none,
                            //
                            
                            ).unwrap();
                        }
                    }

                    termion::event::Key::Right => {
                        if index + 1 != chars.len() { 
                            index += 1;

                        writeln!( self.stdout, 
                            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", 
                            termion::clear::All,

                            termion::cursor::Goto(self.x/2-10,self.y/2),
                            termion::color::Fg(color::LightBlack),
                            termion::color::Bg(color::White),
                            chars[index].name,
                            termion::color::Bg(color::Reset),
                            termion::color::Fg(color::Reset),
                    
                            termion::cursor::Goto(self.x/2-10,self.y/2-3),
                            termion::color::Fg(color::LightBlack),
                            termion::color::Bg(color::White),
                            String::from("Health"),
                            termion::color::Bg(color::Reset),
                            termion::color::Fg(color::Reset),
                            format!(" {}",chars[index].health),
                    
                    
                            termion::cursor::Goto(self.x/2-10,self.y/2-2),
                            termion::color::Fg(color::LightBlack),
                            termion::color::Bg(color::White),
                            String::from("Weapon"),
                            termion::color::Bg(color::Reset),
                            termion::color::Fg(color::Reset),
                            format!(" {:?}",chars[index].weapon),
                    
                            termion::cursor::Goto(self.x/2-10,self.y/2-1),
                            termion::color::Fg(color::LightBlack),
                            termion::color::Bg(color::White),
                            String::from("Armour"),
                            termion::color::Bg(color::Reset),
                            termion::color::Fg(color::Reset),   
                            format!(" {:?}",chars[index].armour),
                    
                            //  TODO complete the listing
                            // name: asd
                            // health
                            // weapon:
                            // Attack: 3, Bonus: Void +3 dmg
                            // armor
                            // protection: 4 Bonus:none,
                            //
                            
                            ).unwrap();
                         }
                    }
                    
                    //enter
                    termion::event::Key::Char('\n') =>{
                        break
                    }
                    _=> {}
                }
            }
            Err(_error) => {}
        }
    }

    return index
}

// this actually doesnt need to be async but igaf
pub async fn upgrade(&mut self,  game: &mut Game ) {

    let mut stack_upgrade: bool = false;
    for key_press in self.stdin.keys() {
    
        // Todo , nice colors depending on the range of xps
        // Todo maybe add undo (hard)
        writeln!( self.stdout, 
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", 
            termion::clear::All,
            termion::cursor::Goto(self.x/2-10, 1),
            "Choose what to upgrade (1-3)",
            termion::cursor::Goto(self.x/2-5, 2),
            "1xp = 0.1 bonus",
            termion::cursor::Goto(self.x/2-5, 3),
            "[S] upgrade by 10: ",
            stack_upgrade,
            termion::cursor::Goto(self.x/2 -5, self.y/2-4),
            format!("Health: {:.2}", game.character.health),

            termion::cursor::Goto(self.x/2 -5, self.y/2-3),
            format!("Attack: {:.2}", game.attack),

            termion::cursor::Goto(self.x/2 -5, self.y/2-2),
            format!("Speed: {:.2}", game.speed),

            termion::cursor::Goto(self.x/2 -5, self.y/2-1),
            format!("XP left: {:.2}", game.xp),
            ).unwrap();

        // this match case is ugly af but ig this is how rust works
        match key_press {

            Ok(_key) => {
                match _key {

                    // health
                    termion::event::Key::Char('1') => {

                        if stack_upgrade {
                            for i in 0..10 {
                                if game.is_upgrade_fundable() {
                                    game.upgrade(crate::game::UpgradeType::Health)
                                }   
                            }
                        }  
                        else {
                            if game.is_upgrade_fundable() {
                                game.upgrade(crate::game::UpgradeType::Health)
                            }   
                        }
                               
                    }
                    // Attack
                    termion::event::Key::Char('2') => {
                        if stack_upgrade {
                            for i in 0..10 {
                                if game.is_upgrade_fundable() {
                                    game.upgrade(crate::game::UpgradeType::Attack)
                                }   
                            }
                        }  
                        else {
                            if game.is_upgrade_fundable() {
                                game.upgrade(crate::game::UpgradeType::Attack)
                            }   
                        }
                    }
                    // Speed
                    termion::event::Key::Char('3') => { 
                        if stack_upgrade {
                            for i in 0..10 {
                                if game.is_upgrade_fundable() {
                                    game.upgrade(crate::game::UpgradeType::Speed)
                                }   
                            }
                        }  
                        else {
                            if game.is_upgrade_fundable() {
                                game.upgrade(crate::game::UpgradeType::Speed)
                            }   
                        }
                     }

                    termion::event::Key::Char('s') => {
                        if stack_upgrade {
                            stack_upgrade = false;
                        } else{
                            stack_upgrade = true;
                        }
                    }
                    // exiting
                    termion::event::Key::Backspace => { break}
                    termion::event::Key::Esc => { break}
                    termion::event::Key::Char('q') => { break}
                    _=> {}
                }

            }
            Err(_error) =>{}
        }
     }  // end of for key press
    


}

}


#[derive(PartialEq)]
enum Tab {
    Armour,
    Potion,
    Sword,
}
struct GuiInventory<'a> {

    pub index: i8,
    pub rows_y: i128,
    pub rows_x: i128,

    pub tab: Tab, 
    potions: HashMap<i128,Potion>,
    swords: HashMap<i128,Sword>,
    armours: HashMap<i128,Armour>,
 
    stdout: &'a mut RawTerminal<Stdout>,
    
    x: u16,
    y: u16,


    pages: i128,
    currentPage: i128,

}


impl<'a> GuiInventory<'a> {

    pub fn new(
        potions: HashMap<i128,Potion>,
        swords: HashMap<i128,Sword>,
        armours: HashMap<i128,Armour>,
        stdout: &'a mut RawTerminal<Stdout>,
        x: u16,
        y: u16,
    ) -> GuiInventory {
            GuiInventory { 
                index: 0, 
                potions: potions,
                swords: swords, 
                armours: armours,
                stdout: stdout,
                x: x,
                y: y,
                rows_y: 0,
                pages:0,
                rows_x: 0,
                currentPage: 0, 
                tab: Tab::Sword,
            }
    }
    
    pub fn render( &mut self ){

        let mut print_string: String = String::new();
        
        print_string.push_str(
            format!("{}{}Page:{}/{}",
                termion::clear::All,
                termion::cursor::Goto(self.x - 10,self.y -5),
                self.currentPage,
                self.pages,
            ).as_str()
        );



        let mut push_string: String = format!("{}", termion::cursor::Goto(1,3), )
            ;
            match self.tab {
                Tab::Sword => {
                    push_string.push_str(format!("{}{}{}{}{}",
                    termion::color::Bg(color::White),
                    termion::color::Fg(color::Black),
                    "Weapons",
                    termion::color::Bg(color::Reset),
                    termion::color::Bg(color::Reset),
                ).as_str())
                }
                Tab::Armour => {
                    push_string.push_str(format!("  {}{}{}{}{}",
                    termion::color::Bg(color::Black),
                    termion::color::Fg(color::White),
                    "Armours",
                    termion::color::Bg(color::Reset),
                    termion::color::Bg(color::Reset),
                ).as_str())
                }
                Tab::Potion => {
                    push_string.push_str(format!("{}{}{}{}{}{}",
                    termion::cursor::Goto(18,3),
                    termion::color::Bg(color::Black),
                    termion::color::Fg(color::White),
                    termion::cursor::Goto(25,3),
                    termion::color::Bg(color::Reset),
                    termion::color::Bg(color::Reset),
                ).as_str())
                }
            }

        print_string.push_str(push_string.as_str());

        // used for margin
        let  console_max_x = self.x - 10;
        let console_max_y = 4;


        let mut current_row_x = 0;
        let mut current_row_y = 0;

        let mut max: bool = false;
        // idk wtf happens here. DONT TOUCH
        for i in 0..self.swords.len() {        

            if (current_row_x * 4 + 4) >= console_max_x {
                if !max {
                    self.rows_x = current_row_x as i128;
                    max = true;
                }
                current_row_y += 1;
                current_row_x = 1;
                self.rows_y += 1;

            }else {
                current_row_x += 1;                
            }
            if i == self.index as usize {

            print_string.push_str(
                format!("{}{}{}{}{}{}",
                                
                termion::cursor::Goto( current_row_x * 4, current_row_y * 4 + 1 + console_max_y),
                String::from("XXX"),
                termion::cursor::Goto(current_row_x * 4, current_row_y * 4 + 2 + console_max_y),
                String::from("XoX"),
                termion::cursor::Goto( current_row_x * 4, current_row_y * 4 + 3 + console_max_y),
                String::from("XXX"),
                ).as_str()
            );
            }
            else {   
            print_string.push_str(
                format!("{}{}{}{}{}{}",
                                
                termion::cursor::Goto( current_row_x * 4, current_row_y * 4 + 1 + console_max_y),
                String::from("  "),
                termion::cursor::Goto(current_row_x * 4, current_row_y * 4 + 2 + console_max_y),
                String::from(" o "),
                termion::cursor::Goto( current_row_x * 4, current_row_y * 4 + 3 + console_max_y),
                String::from("  "),
                ).as_str()
            );
                }
        }

        // printing out the complete inventory
        writeln!(self.stdout, "{}", 
            print_string
        ).unwrap();

    }
    
    pub fn set_index( &mut self, index: i8){
        self.index = index
    }
}
            
// termion::cursor::Goto(self.x/2-10,self.y/2-1),
// termion::color::Fg(color::LightBlack),
// termion::color::Bg(color::White),
// String::from("Armour"),
// termion::color::Bg(color::Reset),
// termion::color::Fg(color::Reset),   
// format!(" {:?}",chars[index].armour),
// ⠀⠀⠘⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ ⡜⠀⠀⠀
// ⠀⠀⠀⠑⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡔⠁⠀⠀⠀
// ⠀⠀⠀⠀⠈⠢⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⠴⠊⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⢀⣀⣀⣀⣀⣀⡀⠤⠄⠒⠈⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠘⣀⠄⠊⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⣿⣿⣿⠃⠀⠀⠀⠀⠀⠀⠀⠀⡜⠘⠀⠀⠀⠀⠈⣿⠀⠁⣿⢳⠁⢈⢰⡦⠈⠀⢻
// ⣿⣿⡏⠀⠀⢀⣀⠀⠀⡀⠀⠀⢀⠀⠀⠁⠀⠀⠈⠉⠀⠀⠀⡘⠀⢰⠛⢠⣴⣄⣌
// ⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⢠⣠⡀⠀⠀⠀⠂⠀⠀⠀⠐⠀⠈⣡⠻⢹⢿
// ⣿⣿⠀⠀⠀⠁⠀⠀⠀⠄⠀⠀⡀⣀⣼⣟⣿⢵⣤⣤⣤⣤⣤⣤⣄⣀⣀⣠⠀⠢⣽
// ⣿⡿⠀⠀⠀⠀⠀⢐⣌⢖⣰⠊⠁⢸⣿⡟⠈⠀⢾⣿⣿⣿⣿⣿⣿⣿⣿⣾⡄⢰⢺
// ⣿⡇⠀⠀⠀⠀⢀⡀⢴⢖⣠⣣⣴⣿⢿⡇⣈⠐⢺⣿⣿⣿⣿⣿⣿⣿⣿⣿⡀⢠⣸
// ⣿⣷⡀⠀⠀⠀⠀⠸⣼⣿⣿⣿⣿⡷⠋⠅⠀⡀⢼⣿⠯⣿⣿⣿⣿⣿⣿⣿⡇⣼⣿
// ⡿⠁⠸⣄⣦⡀⠀⢡⣿⣿⣿⣿⣏⡤⠄⠄⠀⠀⠈⠉⠁⠘⢿⣿⣿⣿⣿⣿⣯⢸⣿
// ⠇⠀⠀⡹⢿⡏⠀⣾⣿⣟⠋⠉⠁⠀⠀⠀⠀⣀⣀⠀⠀⠀⢸⣿⠎⠉⠉⠉⠛⣤⣿
// ⡆⠀⡀⢣⢈⡟⠠⠻⣿⣿⣷⡄⠀⠀⢸⣶⣷⣷⣧⡄⠀⠀⢿⣧⢀⡀⠀⠉⢡⣿⣿
// ⣿⣦⡀⠘⢼⠃⣠⡲⠛⢿⡿⣿⡀⡀⠀⠹⣿⣿⡟⠀⠠⣤⠘⣿⣶⣿⣶⡾⣳⣿⣿
// ⣿⡿⣿⣶⡌⠰⡅⠌⠠⠈⠈⠙⢙⠑⢰⠀⠨⠟⠀⠀⣴⢌⡅⢻⣿⣿⣿⡇⣿⣿⣿
// ⣿⢇⣿⣿⠣⢿⣮⠀⢀⡀⠀⠀⠈⠁⣶⠀⠀⠀⠀⠀⠁⠀⢡⣺⣿⣿⡿⣸⣿⣿⣿
// ⣿⣾⣿⣏⠐⣿⢟⡈⠂⠀⠠⠀⠀⠀⣆⠀⠀⠀⠀⠀⠀⣢⣼⣸⣿⡿⢈⣿⣿⣿⣿
// ⣿⣿⣿⠃⡀⠈⠑⠁⠀⠄⠀⠀⠀⠲⠀⠀⠄⣀⠀⣸⣷⣮⣍⠃⢹⠇⣿⣿⣿⣿⣿
// ⣿⣿⣿⢀⣾⣷⣶⣌⠀⠠⠀⠀⢀⠍⠀⠀⠀⠀⠉⠁⠈⠙⠋⢰⡝⣼⣿⣿⣿⣿⣿
// ⣿⣿⣿⠋⠀⣼⣿⣿⣷⣄⠀⠀⠀⠊⠀⠀⠀⠀⠀⠀⠄⡀⢀⣼⢣⣿⣿⣿⣿⣿⣿
// ⣿⣿⣯⠀⢸⣿⣿⣿⡟⠛⠳⣄⠀⠀⠀⠀⠘⠌⠓⡀⢰⣮⣾⢠⣿⣿⣿⣿⣿⣿⣿
// ⣿⣿⣧⠀⣹⣿⣿⣿⠗⠀⠀⠀⠀⠀⣀⡀⠀⠈⠀⠀⠈⠝⣡⣾⣿⣿⣿⣿⣿⣿⣿
// ⣿⣿⡏⠀⣿⣿⣿⠿⠃⢀⣴⣶⣾⣿⣿⣿⣿⣷⣾⢠⣶⣾⣮⣙⡻⣿⢿⣿⣿⣿⣿
// ⣿⣿⡇⠀⣿⣿⠃⠀⠀⣼⣿⣿⣿⣿⣿⣿⣿⣿⡟⡼⠿⣿⣿⣿⣿⣮⡑⡝⣿⣿⣿