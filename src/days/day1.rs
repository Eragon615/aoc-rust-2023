use crate::Application;

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

    fn part2(self) {}
}
