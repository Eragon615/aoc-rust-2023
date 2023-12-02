use crate::Application;
use std::io::Read;
use std::{collections::BTreeMap, io::Cursor};

impl Application {
    pub fn day1(self) {
        if self.args.part == 1 {
            self.part1();
        } else {
            self.part2();
        }
    }

    fn part1(self) {
        let mut answer = 0;
        for line in self.input {
            let mut first = 0;
            let mut last = 0;
            for char in line.chars() {
                if char.is_numeric() {
                    if first == 0 {
                        first = char as u32 - 48;
                    } else {
                        last = char as u32 - 48;
                    }
                }
            }
            if last == 0 {
                last = first;
            }
            answer += (first * 10) + last;
        }
        println!("{}", answer);
    }

    fn part2(self) {
        let mut answer = 0;
        for line in self.input {
            // if it's stupid and it works, it's not stupid
            let mut map: BTreeMap<u64, u32> = BTreeMap::new();
            let mut line = line.clone();
            for i in 0..line.len() {
                // check for actual numbers first
                if line.as_bytes()[0].is_ascii_digit() {
                    map.insert(i as u64, line.as_bytes()[0] as u32 - 48);
                }
                // now check for all spellings of numbers
                // I got lucky, I searched the input and only 'one' through 'nine' appear
                // this would be much harder otherwise
                if line.find("one") == Some(0) {
                    map.insert(i as u64, 1);
                } else if line.find("two") == Some(0) {
                    map.insert(i as u64, 2);
                } else if line.find("three") == Some(0) {
                    map.insert(i as u64, 3);
                } else if line.find("four") == Some(0) {
                    map.insert(i as u64, 4);
                } else if line.find("five") == Some(0) {
                    map.insert(i as u64, 5);
                } else if line.find("six") == Some(0) {
                    map.insert(i as u64, 6);
                } else if line.find("seven") == Some(0) {
                    map.insert(i as u64, 7);
                } else if line.find("eight") == Some(0) {
                    map.insert(i as u64, 8);
                } else if line.find("nine") == Some(0) {
                    map.insert(i as u64, 9);
                }
                // remove the first letter from the string before continuing
                line.remove(0);
            }
            let mut additive = 0;
            if let Some(first) = map.first_entry() {
                additive += 10 * first.get();
            }
            if let Some(last) = map.last_entry() {
                additive += last.get();
            }
            answer += additive;
        }
        println!("{}", answer);
    }
}
