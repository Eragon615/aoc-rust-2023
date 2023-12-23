use crate::Application;

impl Application {
    pub fn day9(self) {
        let oasis = parse_input(&self.input);
        if self.args.part == 1 {
            self.d9p1(oasis);
        } else {
            self.d9p2(oasis);
        }
    }

    fn d9p1(self, oasis: Vec<Vec<i64>>) {
        let mut answer = 0;
        for num_vec in oasis {
            answer += find_next(&num_vec);
        }
        println!("{answer}");
    }

    fn d9p2(self, oasis: Vec<Vec<i64>>) {
        let mut answer = 0;
        for num_vec in oasis {
            answer += find_prev(&num_vec);
        }
        println!("{answer}");
    }
}

fn parse_input(input: &Vec<String>) -> Vec<Vec<i64>> {
    let mut output = Vec::new();
    for line in input {
        output.push(
            line.clone()
                .split_whitespace()
                .map(|n| n.parse().expect("Number parsing failed"))
                .collect::<Vec<i64>>(),
        );
    }
    return output;
}

fn find_next(input: &Vec<i64>) -> i64 {
    let output;
    let mut deepness = 0;
    let mut working_vec = Vec::new();
    working_vec.push(input.clone());
    loop {
        let new_line = next_row(&working_vec[deepness]);
        deepness += 1;
        working_vec.push(new_line);
        if all_zero(&working_vec[deepness]) {
            break;
        }
    }
    loop {
        let curlast = working_vec[deepness].last().unwrap_or(&0);
        let prevlast = working_vec[deepness - 1]
            .last()
            .expect("We didn't go deep enough");
        let nextnum = curlast + prevlast;
        deepness -= 1;
        if deepness == 0 {
            output = nextnum;
            break;
        }
        working_vec[deepness].push(nextnum);
    }
    return output;
}

fn find_prev(input: &Vec<i64>) -> i64 {
    let output;
    let mut deepness = 0;
    let mut working_vec = Vec::new();
    working_vec.push(input.clone());
    loop {
        let new_line = next_row(&working_vec[deepness]);
        deepness += 1;
        working_vec.push(new_line);
        if all_zero(&working_vec[deepness]) {
            break;
        }
    }
    loop {
        let curfirst = working_vec[deepness].first().unwrap_or(&0);
        let prevfirst = working_vec[deepness - 1]
            .first()
            .expect("Backed into a cop");
        let nextnum = prevfirst - curfirst;
        deepness -= 1;
        if deepness == 0 {
            output = nextnum;
            break;
        }
        working_vec[deepness].insert(0, nextnum);
    }
    return output;
}

fn next_row(input: &Vec<i64>) -> Vec<i64> {
    let mut output = Vec::new();
    for i in 0..(input.len() - 1) {
        output.push(input[i + 1] - input[i]);
    }
    return output;
}

fn all_zero(input: &Vec<i64>) -> bool {
    for i in input {
        if *i != 0 {
            return false;
        }
    }
    return true;
}
