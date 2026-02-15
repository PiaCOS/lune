use clap::{Parser, Subcommand};
use moon::Lune;

mod astro;
mod julian_time;
mod moon;
mod utils;

#[derive(Parser, Debug)]
#[command(name = "lune")]
#[command(version, about = "~ Let's all love Lune ~", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// How the moon is going.
    Current,
    /// Learn about the past and the future.
    Phases,
    /// Learn about the past.
    Prev,
    /// Learn about the future.
    Next,
}

fn main() {
    let cli = Cli::parse();
    let lune = Lune::new();

    match cli.command {
        Some(Commands::Current) => println!("{}", lune.get_current_phase()),
        Some(Commands::Phases) => println!("{}", lune.get_phase_summary()),
        Some(Commands::Prev) => println!("{}", lune.get_prev_phase()),
        Some(Commands::Next) => println!("{}", lune.get_next_phase()),
        None => println!("{}", lune.get_summary()),
    }
}
