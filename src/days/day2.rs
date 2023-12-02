use std::{collections::HashMap, u32};

use crate::Application;

#[derive(Debug)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

type CubeHash = HashMap<u32, Vec<Cubes>>;

impl Application {
    pub fn day2(self) {
        let map = input_to_hash(&self.input);
        if self.args.part == 1 {
            self.d2p1(map);
        } else {
            self.d2p2();
        }
    }

    fn d2p1(self, map: CubeHash) {
        const CUBES_TO_TEST: Cubes = Cubes {
            red: 12,
            green: 13,
            blue: 14,
        };
        let mut answer = 0;

        for (num, entries) in map {
            if entries_are_valid(CUBES_TO_TEST, entries) {
                answer += num;
            }
        }
        println!("{}", answer);
    }

    fn d2p2(self) {}
}

fn input_to_hash(input: &Vec<String>) -> CubeHash {
    let mut output = HashMap::new();
    for line in input {
        let line = line.clone();
        let game_and_cubes: Vec<&str> = line.split(":").collect();
        let game_str = game_and_cubes[0];
        let game_num: u32 = game_str.split_whitespace().collect::<Vec<&str>>()[1]
            .parse()
            .expect("I messed up!");
        let cubes_str = game_and_cubes[1];
        let mut cubes_vec: Vec<Cubes> = Vec::new();
        for set in cubes_str.split(";").collect::<Vec<&str>>() {
            let mut red = 0;
            let mut blue = 0;
            let mut green = 0;
            for color in set.split(",").collect::<Vec<&str>>() {
                let split = color.split_whitespace().collect::<Vec<&str>>();
                let name = split[1];
                let number: u32 = split[0].parse().expect("I screwed up somewhere");
                if name == "red" {
                    red = number;
                } else if name == "green" {
                    green = number;
                } else if name == "blue" {
                    blue = number;
                }
            }
            cubes_vec.push(Cubes { red, green, blue });
        }
        output.insert(game_num, cubes_vec);
    }
    return output;
}

fn entries_are_valid(test: Cubes, entries: Vec<Cubes>) -> bool {
    for entry in entries {
        if test.red < entry.red || test.green < entry.green || test.blue < entry.blue {
            return false;
        }
    }
    return true;
}
