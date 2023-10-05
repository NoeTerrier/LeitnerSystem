mod ltsys_card;
mod ltsys_io;
mod ltsys;

use ltsys_io::open_ltsys;
use clap::{Parser, Subcommand};

use crate::ltsys_io::{create_system, write_to_disk};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional filename to operate on
    filename: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// add a card to the system
    Add {},
    /// remove a card
    Remove { card_name: String },
    /// list all cards
    List {},
    /// draw all cards of the day
    Draw {},
    /// create a new Leitner system
    Create { 
        #[arg(default_value="default.ltsys")]
        filename: String,

        #[arg(default_value="7")]
        boxes_number: u64
    },
}


fn main() {
    let cli = Cli::parse();
    
    // You can check the value provided by positional arguments, or option arguments
    let filename = match cli.filename.as_deref() {
        Some(name) => name,
        None => "default.ltsys",  
    };
    
    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    let _ = match &cli.command {
        Some(Commands::Create { filename , boxes_number}) => {
            create_system(filename, *boxes_number)
        }
        Some(Commands::Add {  }) => {
            let mut ltsys = open_ltsys(&filename.to_string()).unwrap();   
            let _ = ltsys.add_card();
            write_to_disk(&ltsys, &filename.to_string())
        }
        Some(Commands::Remove { card_name }) => {
            let mut ltsys = open_ltsys(&filename.to_string()).unwrap();
            let _ = ltsys.remove_card(card_name);
            write_to_disk(&ltsys, &filename.to_string())
        }
        Some(Commands::List {  }) => {
            open_ltsys(&filename.to_string())
            .unwrap()
            .list_cards()
        }
        Some(Commands::Draw {  }) => {
            let mut ltsys = open_ltsys(&filename.to_string()).unwrap();
            let _ = ltsys.draw_cards();
            write_to_disk(&ltsys, &filename.to_string())
        }
        None => Ok(())
    };
}
