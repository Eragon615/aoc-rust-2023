use std::{thread, usize};

use crate::Application;

#[derive(Debug, Clone)]
struct Almanac {
    seeds: Vec<u64>,
    seeds2soil: Vec<AlmRange>,
    soil2fert: Vec<AlmRange>,
    fert2water: Vec<AlmRange>,
    water2light: Vec<AlmRange>,
    light2temp: Vec<AlmRange>,
    temp2humid: Vec<AlmRange>,
    humid2loc: Vec<AlmRange>,
}

#[derive(Debug, Clone)]
struct AlmRange {
    source: u64,
    destination: u64,
    length: u64,
}

trait AlmTranslate {
    fn alm_translate(self, ranges: &Vec<AlmRange>) -> Self;
}

impl Application {
    pub fn day5(self) {
        let almanac = parse_alm(&self.input);
        if self.args.part == 1 {
            self.d5p1(almanac);
        } else {
            self.d5p2(almanac);
        }
    }

    fn d5p1(self, almanac: Almanac) {
        let mut answer = 0;
        for seed in almanac.seeds {
            let location = seed
                .alm_translate(&almanac.seeds2soil)
                .alm_translate(&almanac.soil2fert)
                .alm_translate(&almanac.fert2water)
                .alm_translate(&almanac.water2light)
                .alm_translate(&almanac.light2temp)
                .alm_translate(&almanac.temp2humid)
                .alm_translate(&almanac.humid2loc);
            if location < answer || answer == 0 {
                answer = location;
            }
        }
        println!("{answer}");
    }

    fn d5p2(self, almanac: Almanac) {
        let mut answer = 0;
        let (tx, rx) = std::sync::mpsc::channel();
        let mut vecofvec = Vec::new();
        for _ in 1..=self.args.thread {
            vecofvec.push(Vec::new());
        }
        for i in 0..almanac.seeds.len() {
            if i % 2 != 0 {
                continue;
            }
            vecofvec[(i / 2) % self.args.thread as usize]
                .push(almanac.seeds[i]..(almanac.seeds[i] + almanac.seeds[i + 1]));
        }
        for i in 0..self.args.thread {
            let newtx = tx.clone();
            let index = i.clone();
            let newvec = vecofvec.clone();
            let newalm = almanac.clone();
            thread::spawn(move || {
                let mut output = 0;
                for range in newvec[index as usize].clone() {
                    for i in range {
                        let location = i
                            .alm_translate(&newalm.seeds2soil)
                            .alm_translate(&newalm.soil2fert)
                            .alm_translate(&newalm.fert2water)
                            .alm_translate(&newalm.water2light)
                            .alm_translate(&newalm.light2temp)
                            .alm_translate(&newalm.temp2humid)
                            .alm_translate(&newalm.humid2loc);
                        if location < output || output == 0 {
                            output = location;
                        };
                    }
                    newtx.send(output).unwrap();
                }
            });
        }
        print!("\n");
        let mut checkin = 1;
        loop {
            print!("\rWaiting on thread {checkin}...");
            if let Ok(output) = rx.try_recv() {
                println!("Thread {checkin}'s lowest was {output}");
                if output < answer || answer == 0 {
                    answer = output;
                }
                checkin += 1;
            }
            if checkin > vecofvec.len() {
                break;
            }
        }
        println!("{answer}");
    }
}

impl AlmTranslate for u64 {
    fn alm_translate(self, ranges: &Vec<AlmRange>) -> Self {
        let mut output = self;
        for range in ranges {
            if self < range.source || self > (range.source + range.length) {
                continue;
            }
            output = range.destination + (self - range.source);
            if output != self {
                break;
            }
        }
        return output;
    }
}

fn parse_alm(input: &Vec<String>) -> Almanac {
    let mut seeds = Vec::new();
    let mut seeds2soil = Vec::new();
    let mut soil2fert = Vec::new();
    let mut fert2water = Vec::new();
    let mut water2light = Vec::new();
    let mut light2temp = Vec::new();
    let mut temp2humid = Vec::new();
    let mut humid2loc = Vec::new();
    let mut section = 9;
    // Parse seeds
    for line in input {
        if line.is_empty() {
            section = 0;
        }
        match section {
            0 => {
                // 0 means we expect a blank line.
                section = 1;
                continue;
            }
            1 => {
                // 1 is expect a new section, read it.
                match line.as_str() {
                    "seed-to-soil map:" => section = 2,
                    "soil-to-fertilizer map:" => section = 3,
                    "fertilizer-to-water map:" => section = 4,
                    "water-to-light map:" => section = 5,
                    "light-to-temperature map:" => section = 6,
                    "temperature-to-humidity map:" => section = 7,
                    "humidity-to-location map:" => section = 8,
                    _ => panic!("Unexpected input line"),
                }
            }
            2 => seeds2soil.push(get_range(line)),
            3 => soil2fert.push(get_range(line)),
            4 => fert2water.push(get_range(line)),
            5 => water2light.push(get_range(line)),
            6 => light2temp.push(get_range(line)),
            7 => temp2humid.push(get_range(line)),
            8 => humid2loc.push(get_range(line)),
            9 => seeds = get_seeds(line),
            _ => panic!("Unexpected section reached."),
        }
    }
    return Almanac {
        seeds,
        seeds2soil,
        soil2fert,
        fert2water,
        water2light,
        light2temp,
        temp2humid,
        humid2loc,
    };
}

fn get_range(input: &String) -> AlmRange {
    let output: Vec<u64> = input
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.parse().expect("Failed to get a range"))
        .collect();
    return AlmRange {
        destination: output[0],
        source: output[1],
        length: output[2],
    };
}

fn get_seeds(input: &String) -> Vec<u64> {
    return input.split(':').collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|n| n.parse().expect("You screwed up parsing the seeds"))
        .collect();
}
