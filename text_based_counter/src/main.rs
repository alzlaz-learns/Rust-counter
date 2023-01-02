#![allow(unused)]


use std::env;
use std::io;

mod loop_logic;
use crate::loop_logic::{MainLoopCommands, main_loop_match};

extern crate counter_components;
use::counter_components::*;


const SAVE_FILE: &str = r"..\cli_counter\info\save_file.json";
fn main() {

    // let mut f = file_handler::FileHandler::load_profiles(SAVE_FILE);
    // println!("{:?}", f);
    // f.save(SAVE_FILE, &f);

    println!("Starting up TEXT BASED SCORE_CARD_APP");
    loop{
        
        let mut u_input = String::new();
        io::stdin()
            .read_line(&mut u_input)
            .expect("Failed to read line");
        let inp = MainLoopCommands::from_str(u_input.trim());
        // enum_match(inp);
        match inp {
            Ok(v) => main_loop_match(v),
            Err(_) => println!("invalid input")
        }
    
    }
}

/*
    /*
        Outer Logic loop:
            Take user input to:
                create a new Score card
                Load existing Score card 
                display existing Score cards
                Access help for command line prompts.
                exit program
    */
    loop{
        println!("Testing loop input");
        let mut u_input = String::new();
        io::stdin()
        .read_line(&mut u_input)
        .expect("Failed to read line");

        let inp = LoopCommands::from_str(u_input.trim()).unwrap();
        enum_match(inp);
    
    }

*/