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
    Remove {},
    /// list all cards
    List {},
    /// draw all cards of the day
    Draw {},
    /// create a new Leitner system
    Create {},
}


fn main() {
    let cli = Cli::parse();
    
    
    // You can check the value provided by positional arguments, or option arguments
    let filename = match cli.filename.as_deref() {
        Some(name) => name,
        None => "default.ltsys",  
    };
    
    println!("Value for name: {filename}");
    let mut ltsys = open_ltsys(&filename.to_string()).unwrap();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    let _ = match &cli.command {
        Some(Commands::Add {  }) => {
            let _ = ltsys.add_card();
            write_to_disk(&ltsys, &filename.to_string())
        }
        Some(Commands::Create {  }) => {
            create_system()
        }
        Some(Commands::Remove {  }) => {
            let _ = ltsys.remove_card();
            write_to_disk(&ltsys, &filename.to_string())
        }
        Some(Commands::List {  }) => {
            ltsys.list_cards()
        }
        Some(Commands::Draw {  }) => {
            let _ = ltsys.draw_cards();
            write_to_disk(&ltsys, &filename.to_string())
        }
        None => Ok(())
    };
}
