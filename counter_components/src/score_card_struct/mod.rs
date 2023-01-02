#![allow(unused)]

use serde_derive::{Deserialize, Serialize};

use crate::counter_struct::Counter;

#[derive(Deserialize, Serialize, Debug)]

//definition of scorecard struct
pub struct ScoreCard{
    pub name: String,
    pub player_list: Vec<Counter>,
}

impl ScoreCard{
    //create a new counter struct with a player name.
    pub fn new(input_name: impl Into<String>) -> Self{
        ScoreCard{
            name: input_name.into(),
            player_list: Vec::new()
        }
    }
}

//instantiation of a default struct if there is no scorecard name.
//probably will remove
impl Default for ScoreCard{
    fn default() -> Self {
        ScoreCard{
            name: "Game".to_string(),
            player_list: Vec::new().to_owned()
        }
    }
}

impl ScoreCard{
    //didnt think this through it didnt make sense add a premade counter to the score card.
    // needs to be score card makes a counter struct to add.
    pub fn add_counter(&mut self, counter: Counter){
        self.player_list.push(counter);
    }

    pub fn add(&mut self, name: &str){
        let counter = Counter::new(name);
        self.player_list.push(counter);
    }

    pub fn remove(&mut self, pos: usize){
        self.player_list.remove(pos);
    }

    pub fn get_details(&mut self){
        println!("{}", self.name);
        println!("{:?}", self.player_list);
    }
}

// specialized ScoreCard methods for text based app.
// returns option type
impl ScoreCard{
    //finds the name in the player_list vector
    pub fn find_by_name(&mut self, name: &str) -> Option<usize>{
        let index_element = self.player_list
                .iter()
                .position(|x| x.player_name == name);
        // println!("{:?}", index_element);        

        index_element
    }
}
