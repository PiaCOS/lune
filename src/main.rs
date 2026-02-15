use clap::Parser;

use moon::Lune;

mod astro;
mod julian_time;
mod moon;
mod utils;

/// Let's all love Lune.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Learn about the moon.
    #[arg(short, long, action)]
    summary: bool,

    /// How the moon is going.
    #[arg(short, long, action)]
    current: bool,

    /// Learn about the past and the future.
    #[arg(short, long, action)]
    phases: bool,
}

fn main() {
    let args = Args::parse();

    let lune = Lune::new();

    if args.current {
        println!("{}", lune.get_current_phase());
    } else if args.phases {
        println!("{}", lune.get_phase_summary());
    } else {
        println!("{}", lune.get_summary());
    }
}
