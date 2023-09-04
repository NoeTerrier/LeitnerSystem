use std::io::{self, Write};

use crate::{
    ltsys_io::{ask_string, open_ltsys, write_to_disk},
    Card, LeitnerSystem,
};

pub fn add_card() -> Result<(), String> {
    // ask file name
    let filename = ask_string("filename", "default.ltsys")?;
    let mut ltsys = open_ltsys(&filename)?;

    // ask attributes of the card
    let name = ask_string("name", "new_card")?;
    let question = ask_string("question", "empty question")?;
    let answer = ask_string("answer", "empty answer")?;
    let card = Card {
        name: name.clone(),
        question,
        answer,
    }; // name needs to be cloned to be used in print later

    // sort by frequence and insert card
    ltsys.boxes.sort_by(|a, b| a.frequence.cmp(&b.frequence));
    ltsys.boxes.get_mut(0).unwrap().cards.push(card);

    write_to_disk(&ltsys, &filename)?;
    print!("> Card [{}] added successfully\n", name);
    Ok(())
}

pub fn remove_card() -> Result<(), String> {
    // ask file name
    let filename = ask_string("filename", "default.ltsys")?;
    let mut ltsys = open_ltsys(&filename)?;

    let name = ask_string("card name", "")?;

    for b in ltsys.boxes.iter_mut() {
        b.cards.retain(|c| !c.name.eq(&name));
    }
    write_to_disk(&ltsys, &filename)?;
    print!("> Card [{}] removed successfully\n", name);
    Ok(())
}

pub fn list_cards() -> Result<(), String> {
    // ask file name
    let filename = ask_string("filename", "default.ltsys")?;
    let ltsys = open_ltsys(&filename)?;

    for b in ltsys.boxes {
        print!("+-------------------------------+\n");
        print!("| Box {:3}: all {} days\n", b.name, b.frequence);
        for c in b.cards {
            print!(
                "| [{}]\n|  >question: {}\n|  >answer: {}\n",
                c.name, c.question, c.answer
            );
        }
    }
    print!("+-------------------------------+\n");

    Ok(())
}

pub fn draw_cards() -> Result<(), String> {
    // ask file name
    let filename = ask_string("filename", "default.ltsys")?;
    let ltsys = open_ltsys(&filename)?;

    let mut new_ltsys = ltsys.clone();

    let days_elapsed = ltsys.start_time.elapsed().unwrap().as_secs() / 86400 + 1;

    let mut i = 0;
    let mut no_card = true;
    for b in ltsys.boxes {
        if days_elapsed % b.frequence != 0 {
            continue;
        }

        for c in b.cards {
            no_card = false;
            let correct = anwser_correctly(&c)?;
            move_to_correct_box(&mut new_ltsys, &c, i, correct)?;
        }

        i += 1;
    }

    if no_card {
        print!("> There is no card today!\n");
    }

    write_to_disk(&new_ltsys, &filename)
}

fn anwser_correctly(card: &Card) -> Result<bool, String> {
    print!("[{}]\nquestion: {}", card.name, card.question);
    io::stdout().flush().or(Err("error on stdout"))?;

    let mut string = String::new();
    io::stdin()
        .read_line(&mut string)
        .or(Err("error reading line"))?;

    print!("anwser: {}\n", card.answer);
    print!("success ? (y/n): ");
    io::stdout().flush().or(Err("error on stdout"))?;
    io::stdin()
        .read_line(&mut string)
        .or(Err("error reading line"))?;

    Ok(string.trim().ends_with("y"))
}

fn move_to_correct_box(
    new_ltsys: &mut LeitnerSystem,
    card: &Card,
    box_index: usize,
    correct: bool,
) -> Result<(), String> {
    if correct && box_index == new_ltsys.boxes.len() - 1 {
        print!("CONGRATS! You succefully learned card [{}]\n", card.name);
        return Ok(());
    } else if !correct && box_index == 0 {
        return Ok(());
    }

    let delta = if correct { 1 } else { -1 };

    let current_box = match new_ltsys.boxes.get_mut(box_index) {
        Some(b) => b,
        None => {
            return Err("box index out of bound".to_string());
        }
    };

    let index = match current_box.cards.iter().position(|c| c.name.eq(&card.name)) {
        Some(i) => i,
        None => {
            return Err(format!("no card with name {}", card.name));
        }
    };

    let card = current_box.cards.remove(index);

    match new_ltsys
        .boxes
        .get_mut(((box_index as i64) + delta) as usize)
    {
        Some(b) => b.cards.push(card),
        None => {
            return Err(format!(
                "cannot find box at index {}",
                (box_index as i64) + delta
            ));
        }
    }

    Ok(())
}
