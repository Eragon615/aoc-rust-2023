use crate::Application;

impl Application {
    pub fn day9(self) {
        let oasis = parse_input(&self.input);
        if self.args.part == 1 {
            self.d9p1(oasis);
        } else {
            self.d9p2();
        }
    }

    fn d9p1(self, oasis: Vec<Vec<i64>>) {
        let mut answer = 0;
        for num_vec in oasis {
            answer += find_next(&num_vec);
        }
        println!("{answer}");
    }

    fn d9p2(self) {}
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
    todo!()
}
