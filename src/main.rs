mod args;
mod days;

use args::Args;
use clap::Parser;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

pub struct Application {
    pub args: Args,
    pub input: Vec<String>,
}

fn main() {
    let app = setup();
    match app.args.day {
        1 => app.day1(),
        2 => app.day2(),
        3 => app.day3(),
        4 => app.day4(),
        5 => app.day5(),
        6 => app.day6(),
        7 => app.day7(),
        8 => app.day8(),
        9 => app.day9(),
        10 => app.day10(),
        _ => println!("It is not yet the time..."),
    };
}

fn setup() -> Application {
    let args = Args::parse();
    let filename = match args.sample {
        false => format!("input/input{}.txt", &args.day),
        true => format!("sample/input{}.txt", &args.day),
    };
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    let input: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    Application { args, input }
}
