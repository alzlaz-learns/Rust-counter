use std::{process::exit, io};

extern crate counter_components;
use::counter_components::*;
use counter_components::{score_card_struct::ScoreCard, counter_struct::Counter};


#[derive(Debug)]
pub enum MainLoopCommands{
    /*
        Enums representing commands a user 
    */
    NewGame,
    Exit,
    Help,
    Display,
}

#[derive(Debug)]
pub struct InputError;
pub struct InvalidArgCount;

pub enum ErrorEnums {
    InputError,
    InvalidArgCount
}

impl MainLoopCommands {
    pub fn from_str(s: &str,) -> Result<MainLoopCommands, InputError>{
        match s {
            "-new"|"-n" => Ok(MainLoopCommands::NewGame),
            "-exit"|"-e" => Ok(MainLoopCommands::Exit),
            "-help"|"-h" => Ok(MainLoopCommands::Help),
            "-display"|"-d" => Ok(MainLoopCommands::Display),
            _ => Err(InputError),
        }
    }
}

pub fn main_loop_match(commands: MainLoopCommands){
    match commands {
        MainLoopCommands::NewGame => {
            println!("Starting a new game");
            new_game();
        },
        MainLoopCommands::Exit => {
            println!("Thank you for playing"); 
            exit(0);
        },
        MainLoopCommands::Help => {
            println!("In game help section listing");
            println!("Options:
            -n or -New: To Start a new game loop.
            \t e.g -n
            \n -e or -Exit: To end the game.
            \t e.g -e 
            \n -d or -Display: to show saved score cards.
            \t currently not an option. To be worked upon.
            ");
        },
        MainLoopCommands::Display => {
            println!("Displaying saved cards");
            /*
                To be inmplemented if decided to 
            */
            // let mut f = file_handler::FileHandler::load_profiles(SAVE_FILE);
            // println!("{:?}", f);
            // f.save(SAVE_FILE, &f);
        },
    }
}


fn new_game(){
    println!("Enter a name for your score card. (Just hit ENTER for default)");
    let mut u_input = String::new();
    io::stdin()
        .read_line(&mut u_input)
        .expect("Failed to read line");

    let mut score_card_struct: ScoreCard;
    
    if u_input.trim().len() > 0
    {
        score_card_struct = ScoreCard::new(u_input.trim());
        // score_card_struct.get_details();
    }else{
        score_card_struct = ScoreCard::default();
        // score_card_struct.get_details();
    }

    game_loop(&mut score_card_struct);
    println!("exiting game");
}

pub enum GameLoopCommands{
    /*
        Enums representing the user commands
        for within a game. 
    */
    AddPlayer,
    RemovePlayer,
    Help,
    EndGame,
    Increment,
    Decrement,
}

impl GameLoopCommands{
    pub fn from_str(s: &str,) -> Result<(GameLoopCommands, &str), ErrorEnums>{

        /*
            TODO: Handle the errors correctly for arg count vs invalid input.
        */
        let vec: Vec<&str> = s.split(' ').collect();
        match vec.len() {
            2 => {
                match vec[0] {
                    "-Add"|"-a" => Ok((GameLoopCommands::AddPlayer, vec[1])),
                    "-Remove"|"-r" => Ok((GameLoopCommands::RemovePlayer, vec[1])),
                    "-Increment"|"-i" => Ok((GameLoopCommands::Increment, vec[1])),
                    "-Decrement"|"-d" => Ok((GameLoopCommands::Decrement, vec[1])),
                    _ => Err(ErrorEnums::InputError),
                }
            },
            1 => {
                match vec[0] {
                    "-Help"|"-h" => Ok((GameLoopCommands::Help, "")),
                    "-End"|"-e" => Ok((GameLoopCommands::EndGame, "")),
                    _ => Err(ErrorEnums::InputError),
                }
            },
            _ => Err(ErrorEnums::InvalidArgCount) 
        }

    }
}

fn game_loop_match(commands: (GameLoopCommands, &str), score_card_struct: &mut ScoreCard){

    match commands {
        (GameLoopCommands::AddPlayer, player_name) => {
            println!("adding a player: {}", player_name);
            score_card_struct.add(player_name);
            
        }, 
        (GameLoopCommands::RemovePlayer, player_name) => {
            println!("Removing a player: {}", player_name);
            let pos = score_card_struct.find_by_name(player_name);
            
            if pos == None {
                println!("Could not find player: {}", player_name);
                return;
            }
            score_card_struct.remove(pos.unwrap());
            
        },
        (GameLoopCommands::Help, player_name) => {
            println!("In game help section listing");
            println!("Options:
            -a or -Add (player name): To add a player.
            \t e.g -a p1
            \n -r or -Remove (player name): To remove a player.
            \t e.g -r p1
            \n -i to -Increment (player name): to increase a player's score.
            \t e.g -i p1
            \n -i to -Decrement (player name): to decrease a player's score.
            \t e.g -d p1
            \n -e or -End: To end the game.
            \t e.g -e 
            ");

        },
        (GameLoopCommands::EndGame, player_name) => {
            return;
            
        },
        (GameLoopCommands::Increment, player_name) => {
            println!("Increment a players score: {}", player_name);
            let pos = score_card_struct.find_by_name(player_name);
            
            match pos {
                Some(pos) => {
                    &score_card_struct.player_list[pos].increment();
                 },
                None => {
                    println!("Could not find player: {}", player_name);
                    return;
                }
            }            
        },
        (GameLoopCommands::Decrement, player_name) => {
            println!("Decrement a players score: {}", player_name);
            let pos = score_card_struct.find_by_name(player_name);
            
            match pos {
                Some(pos) => {
                    &score_card_struct.player_list[pos].decrement();
                 },
                None => {
                    println!("Could not find player: {}", player_name);
                    return;
                }
            }
        },
        
    }
}


fn game_loop( score_card_struct:  &mut ScoreCard){
    println!("starting game loop");
    loop{
        // clearscreen::clear().expect("failed to clear screen");
        let mut u_input = String::new();
        io::stdin()
            .read_line(&mut u_input)
            .expect("Failed to read line");

        let inp = GameLoopCommands::from_str(u_input.trim());
        match inp {
            
            Ok((GameLoopCommands::EndGame, "")) => {
               game_loop_match((GameLoopCommands::EndGame, ""), score_card_struct);
               break;
            },
            Ok(v) => game_loop_match(v, score_card_struct),  
            Err(ErrorEnums::InputError) => println!("invalid input"),
            Err(ErrorEnums::InvalidArgCount) => println!("Invalid arg count"),
        }
        score_card_struct.get_details();
    }
}