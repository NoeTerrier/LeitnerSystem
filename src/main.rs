mod ltsys_card;
mod ltsys_io;

use ltsys_card::{add_card, draw_cards, list_cards, remove_card};
use ltsys_io::create_system;
use serde::{Deserialize, Serialize};
use std::env;
use std::time::SystemTime;

type Action = fn() -> Result<(), String>;

static COMMANDS: &'static [(&'static str, Action, &'static str)] = &[
    ("help", print_help, "displays this message"),
    ("add", add_card, "add a card to the system"),
    ("remove", remove_card, "remove a card"),
    ("list", list_cards, "list all cards"),
    ("draw", draw_cards, "draw all cards of the day"),
    ("create", create_system, "create a new Leitner system"),
];

#[derive(Serialize, Deserialize, Clone)]
pub struct LeitnerSystem {
    start_time: SystemTime,
    boxes: Vec<Box>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Card {
    name: String,
    question: String,
    answer: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Box {
    name: String,
    frequence: u64,
    cards: Vec<Card>,
}

fn main() {
    let arg = env::args().skip(1).next();

    match arg {
        Some(cmd) => {
            for (name, fun, _) in COMMANDS {
                if cmd.eq(name) {
                    match fun() {
                        Ok(()) => (),
                        Err(e) => print!("> /!\\ {}\n", e),
                    }
                    return;
                }
            }
            print!("> /!\\ Command doesn't exist\n");
            let _ = print_help();
            return;
        }
        None => {
            let _ = print_help();
            return;
        }
    }
}

fn print_help() -> Result<(), String> {
    print!("=== HELP ===\nAvailable commands:\n");
    for (name, _, description) in COMMANDS {
        print!("  {:10}: {}\n", name, description);
    }
    print!("============\n");
    Ok(())
}
