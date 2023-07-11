use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;

use daily_log::{config::get_config, log::open_log};

/// A CLI for recording daily tasks, notes, and thoughts.
#[derive(Debug, Parser)]
#[clap(name = "log")]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Append today's log to monthly log
    ///
    /// This command will append the saved text for today into the log
    /// for the month. If you have any unfinished tasks, it will move
    /// them to a new file for tomorrow.
    #[clap(visible_alias = "c")]
    Close,
    /// Show the log for this month
    ///
    /// This command brings up the log file for this month. You can choose
    /// to only view the log or open in an editor to make changes.
    #[clap(visible_alias = "s")]
    Show {
        /// Open the monthly log in your $EDITOR.
        #[clap(short, long)]
        edit: bool,
    },
    /// Start a new log for today from a template
    ///
    /// This command creates a file in your log folder for today (if it
    /// doesn't already exist) and inserts a starter template. It then
    /// opens the file in your $EDITOR.
    #[clap(visible_alias = "t")]
    Today {
        /// Reset the date for the log to today's date
        #[clap(short, long)]
        reset: bool,
    },
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    let config = get_config()?;
    match cli.command {
        Command::Close => println!("Close it!"),
        Command::Show { edit: _ } => println!("Show logs for month."),
        Command::Today { reset } => open_log(config, reset)?,
    }
    Ok(())
}
