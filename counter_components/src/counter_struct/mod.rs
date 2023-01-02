
#![allow(unused)]
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
//definition of a count struct.
pub struct Counter{
    pub player_name: String,
    pub count: u16,
}

//instantiation of a default struct if there is no player name.
impl Default for Counter{
    fn default() -> Self {
        Counter{
            player_name: "Player".to_string(),
            count: 0,
        }
    }
}

//general purpose struct methods.
impl Counter{
    //create a new counter struct with a player name.
    pub fn new(name: impl Into<String>) -> Self {
        Counter { 
            count: 0, 
            player_name: name.into(),
        }
    }

    //increase a counter value
    pub fn increment(&mut self){
        self.count += 1;
    }

    //decrease a counter value
    pub fn decrement(&mut self){
        self.count -= 1;
    }

    fn get_name(&self) -> &str{
        &self.player_name
    }

    fn get_count(&self) -> u16 {
        self.count
    }

}

// specialized Counter methods for text based app.
impl Counter {
    //quick display function
    pub fn present(self){
        println!("Player: {} \nScore: {}", self.get_name(), self.get_count());
    }
}


