use std::collections::HashMap;

use crate::Application;

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

type DesertMap = HashMap<String, Node>;

impl Application {
    pub fn day8(self) {
        let (directions, map) = parse_map(&self.input);
        if self.args.part == 1 {
            self.d8p1(directions, map);
        } else {
            self.d8p2(directions, map);
        }
    }

    fn d8p1(self, directions: Vec<Direction>, map: DesertMap) {
        let mut answer = 0;
        let mut location = "AAA".to_string();
        loop {
            let mut stop = false;
            for d in &directions {
                location = follow_map(&map, location, &d);
                answer += 1;
                if location == "ZZZ".to_string() {
                    stop = true;
                }
            }
            if stop {
                break;
            }
        }
        println!("{answer}");
    }

    fn d8p2(self, directions: Vec<Direction>, map: DesertMap) {
        let mut answer = 0;
        let mut distances = Vec::new();
        let locations: Vec<String> = map
            .keys()
            .filter(|k| k.ends_with('A'))
            .map(|k| k.to_owned())
            .collect();
        for location in locations {
            let mut curloc = location.clone();
            let mut distance = 0;
            loop {
                let mut stop = false;
                for d in &directions {
                    curloc = follow_map(&map, curloc, &d);
                    distance += 1;
                    if check_end(&curloc) {
                        stop = true;
                    }
                }
                if stop {
                    distances.push(distance);
                    break;
                }
            }
        }
        println!("{:?}", distances);
    }
}

fn parse_map(input: &Vec<String>) -> (Vec<Direction>, DesertMap) {
    let mut outmap = HashMap::new();
    let mut outdir = Vec::new();

    for char in input[0].chars() {
        let direction = match char {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!(),
        };
        outdir.push(direction);
    }

    for i in 2..input.len() {
        let key = input[i].clone().split(" =").collect::<Vec<&str>>()[0].to_string();
        let right_hand_side = input[i].split('=').collect::<Vec<&str>>()[1]
            .strip_prefix(" (")
            .expect("Missing the opening (")
            .strip_suffix(')')
            .expect("Missing the closing )")
            .split(", ")
            .collect::<Vec<&str>>();
        let node = Node {
            left: right_hand_side[0].to_string(),
            right: right_hand_side[1].to_string(),
        };
        let _ = outmap.insert(key, node);
    }

    return (outdir, outmap);
}

fn follow_map(map: &DesertMap, key: String, direction: &Direction) -> String {
    let node = map.get(&key).expect("Looked up a node that doesn't exist");
    let output = node.left_or_right(direction);
    return output;
}

fn check_end(input: &String) -> bool {
    let mut data = input.clone();
    if let Some(letter) = data.pop() {
        if letter == 'Z' {
            return true;
        }
    }
    return false;
}

impl Node {
    fn left_or_right(&self, direction: &Direction) -> String {
        let output = match direction {
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        };
        return output.to_owned();
    }
}

fn vec_lcm(input: Vec<u64>) -> u64 {
    // I need to come back and write this,
    // instead of just using Wolfram Alpha
    todo!()
}
