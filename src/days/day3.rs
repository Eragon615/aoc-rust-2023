use std::ops::RangeInclusive;

use crate::Application;

struct Diagram {
    symbols: Vec<Symbol>,
    numbers: Vec<Number>,
}

#[derive(Debug, Clone)]
struct Symbol {
    glyph: char,
    x_coord: usize,
    y_coord: usize,
}

#[derive(Debug, Clone)]
struct Number {
    value: u64,
    length: usize,
    x_coord: usize, // The starting coord, plus the length if more than one digit
    y_coord: usize,
}

impl Application {
    pub fn day3(self) {
        let diagram = map_input(&self.input);
        if self.args.part == 1 {
            self.d3p1(diagram);
        } else {
            self.d3p2();
        }
    }

    fn d3p1(self, diagram: Diagram) {
        let mut number_list = Vec::new();
        for symbol in &diagram.symbols {
            number_list.append(&mut find_adjacent_numbers(&diagram.numbers, symbol));
        }
        let mut answer = 0;
        for number in number_list {
            answer += number.value;
        }
        println!("{answer}");
    }

    fn d3p2(self) {}
}

fn map_input(input: &Vec<String>) -> Diagram {
    let mut symbols_out: Vec<Symbol> = Vec::new();
    let mut numbers_out: Vec<Number> = Vec::new();
    for y in 0..input.len() {
        let line = input[y].chars().collect::<Vec<char>>();
        let mut advencer = 0;
        for x in 0..line.len() {
            if advencer > 0 {
                advencer -= 1;
                continue;
            }
            if line[x] == '.' {
                continue;
            }
            if line[x].is_numeric() {
                let mut num_str = Vec::new();
                num_str.push(line[x].clone());
                check_next(&line, x, &mut num_str);
                advencer = num_str.len() - 1; // if we got more than one digit, skip them
                let num = num_str
                    .iter()
                    .collect::<String>()
                    .parse()
                    .expect("Oops, I screwed something up");
                numbers_out.push(Number {
                    value: num,
                    length: num_str.len(),
                    x_coord: x,
                    y_coord: y,
                })
            } else {
                symbols_out.push(Symbol {
                    glyph: line[x].clone(),
                    x_coord: x,
                    y_coord: y,
                })
            }
        }
    }
    return Diagram {
        symbols: symbols_out,
        numbers: numbers_out,
    };
}

fn check_next(line: &Vec<char>, pos: usize, num_str: &mut Vec<char>) -> () {
    if line.len() <= pos + 1 {
        return; // make sure we don't index out of bounds
    }
    if line[pos + 1].is_numeric() {
        num_str.push(line[pos + 1].clone());
        check_next(&line, pos + 1, num_str);
    }
}

fn find_adjacent_numbers(num_list: &Vec<Number>, symbol: &Symbol) -> Vec<Number> {
    let output: Vec<Number> = num_list
        .clone()
        .into_iter()
        .filter(|number| {
            (minus_one_or(symbol.y_coord)..=(symbol.y_coord + 1)).contains(&number.y_coord)
        })
        .filter(|number| number.range_check(minus_one_or(symbol.x_coord)..=(symbol.x_coord + 1)))
        .collect();
    return output;
}

impl Number {
    fn range_check(&self, range: RangeInclusive<usize>) -> bool {
        for sym_coord in range {
            for num_coord in self.x_coord..(self.x_coord + self.length) {
                if num_coord == sym_coord {
                    return true;
                }
            }
        }
        return false;
    }
}

fn minus_one_or(input: usize) -> usize {
    if input == 0 {
        return 0;
    }
    return input - 1;
}
