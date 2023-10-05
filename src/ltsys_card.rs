use serde::{Deserialize, Serialize};
use std::{
    io::{self, Write},
    time::SystemTime,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Card {
    pub name: String,
    pub question: String,
    pub answer: String,
    pub answered_date: SystemTime, // last time it was correctly answered 
    pub box_number: u64, // answer this question when the day is more than answered_date + 2**box_number
}

impl Card {
    pub fn move_to_correct_box(
        &mut self,
        boxes_number: u64,
        correct: bool,
    ) -> Result<(), String> {
        if correct && self.box_number == boxes_number {
            print!("CONGRATS! You succefully learned card [{}]\n", self.name);
            return Ok(());
        } else if !correct && self.box_number == 0 {
            return Ok(());
        }
    
        self.answered_date = SystemTime::now();
        self.box_number += 1;
    
        Ok(())
    }

    pub fn anwser_correctly(&self) -> Result<bool, String> {
        print!("[{}]\n> question: {}", self.name, self.question);
        io::stdout().flush().or(Err("error on stdout"))?;
    
        let mut string = String::new();
        io::stdin()
            .read_line(&mut string)
            .or(Err("error reading line"))?;
    
        print!("> anwser: {}\n", self.answer);
        print!("> success ? (y/n): ");
        io::stdout().flush().or(Err("error on stdout"))?;
        io::stdin()
            .read_line(&mut string)
            .or(Err("error reading line"))?;
    
        Ok(string.trim().ends_with("y"))
    }
}
