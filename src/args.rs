use clap::{value_parser, Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Increase verboseness for troubleshooting.
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
    /// The day of the challenge
    #[arg(value_parser = value_parser!(u32).range(1..=25))]
    pub day: u32,
    /// Part one or part 2
    #[arg(value_parser = value_parser!(u32).range(1..=2))]
    pub part: u32,
    /// Use the sample input instead
    #[arg(short, long)]
    pub sample: bool,
    /// The number of threads to use, if programmed for multithreading
    #[arg(short, long, value_parser = value_parser!(u32).range(1..=32), default_value_t = 1)]
    pub thread: u32,
}
