use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::{ltsys_card::Card, ltsys_io::ask_string};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LeitnerSystem {
    pub cards: Vec<Card>,
    pub boxes_number: u64,
}

impl LeitnerSystem {
    pub fn draw_cards(&mut self) -> Result<(), String> {
        let mut no_card = true;
        let base: u64 = 2;
        for c in self.cards.iter_mut() {
            if c.answered_date.elapsed().unwrap().as_secs() / 86400 + 1 >= base.pow(c.box_number as u32 - 1) {
                no_card = false;
                let correct = c.anwser_correctly()?;
                c.move_to_correct_box(self.boxes_number, correct)?;
            }
        }

        if no_card {
            print!("> There is no card today!\n");
        }

        Ok(())
    }

    pub fn add_card(&mut self) -> Result<(), String> {    
        // ask attributes of the card
        let name = ask_string("name", "new_card")?;
        let question = ask_string("question", "empty question")?;
        let answer = ask_string("answer", "empty answer")?;
        let card = Card {
            name: name.clone(),
            question,
            answer,
            answered_date: SystemTime::now(),
            box_number: 1,
        }; // name needs to be cloned to be used in print later
    
        // sort by frequence and insert card
        self.cards.push(card);
    
        print!("> Card [{}] added successfully\n", name);
        Ok(())
    }

    pub fn remove_card(&mut self, name: &String) -> Result<(), String> {
        self.cards.retain(|c| !c.name.eq(name));
    
        print!("> Card(s) [{}] removed successfully\n", name);
        Ok(())
    }

    pub fn list_cards(&self) -> Result<(), String> {
        let mut boxes: Vec<Vec<Card>> = vec![vec![]; self.boxes_number as usize];
    
        for c in self.cards.iter() {
            boxes[c.box_number as usize - 1].push(c.clone());
        }
    
        let mut n = 0;
        let base: u64 = 2;
        for b in boxes {
            n += 1;
            print!("+-------------------------------+\n");
            print!("| Box {:3}: all {} days\n", n, base.pow(n as u32 - 1));
            for c in b {
                print!(
                    "| [{}]\n|  >question: {}\n|  >answer: {}\n",
                    c.name, c.question, c.answer
                );
            }
        }
        print!("+-------------------------------+\n");
    
        Ok(())
    }
}
