use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};
use colored::Colorize;
use tracing::{warn, info};


fn main() {
    println!("{} Lisp Version 0.0.1", format!("Sauce").red());
    println!("Press Ctrl+c or type \"q\" to Exit\n");
    
    ui().map_err(|err| println!("{:?}", err)).ok();
}

fn ui() -> Result<()>  {
    let mut rl = Editor::<()>::new()?;
    if rl.load_history("./.sauce-history.txt").is_err() {
        println!("No history found.");
    }

    loop {
        let input = rl.readline(&format!("{}>", "s".red()));
        match input {
            Ok(line) => {
                rl.add_history_entry::<&str>(line.as_str().as_ref());
                if line.trim().to_lowercase() == "q" {
                    toodles();
                    break;
                }

            }
            Err(ReadlineError::Interrupted) => {
                info!("CTRL-C");
                toodles();
                break;
            }
            Err(ReadlineError::Eof) => {
                info!("CTRL-D");
                toodles();
                break;
            }
            Err(err) => {
                warn!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("./.sauce-history.txt")?;
    Ok(())
}

fn toodles() {
    println!("Have a {} Day", "Saucy".red())
}