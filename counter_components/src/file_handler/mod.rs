#![allow(unused)]


use std::fs;
use std::fs::{File};
use std::io::{self, prelude::*, BufReader};
use std::path::Path;
use std::error::Error;
use serde_derive::{Deserialize, Serialize};


use crate::score_card_struct::ScoreCard;

#[derive(Deserialize, Serialize, Debug)]

//file loading and handling 
pub struct FileHandler {
    pub score_card_list: Vec<ScoreCard>,
}

impl FileHandler{

    pub fn new() -> Self {
        FileHandler { 
            score_card_list: Vec::new(),
        }
    }
    
    pub fn load_profiles(path: &str ) -> Self {
            match load_cards(path) {
                Ok(res) => res,
                Err(_) => {
                    println!("there was some kind of whoopsies");
                    Self::new() //Need to handle this more appropriately.
                },
            }
        }
    
    pub fn add(&mut self, score_card: ScoreCard){
        self.score_card_list.push(score_card);
    }

    pub fn save<P: AsRef<Path>>(&self, path: P, sc_list: &FileHandler )-> Result<(), std::io::Error>{
        let j = serde_json::to_string(&sc_list)?;
        // Print, write to a file.
        println!("{}", &j);
        fs::write(
            path,
            &j
        )?;
    
        Ok(())
        
    }
}


fn load_cards<P: AsRef<Path>>(path: P) -> Result<FileHandler, Box<dyn Error>>{
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}

//file loading and handling
//region end