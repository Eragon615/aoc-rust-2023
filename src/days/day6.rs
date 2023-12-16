use crate::Application;

#[derive(Debug)]
struct RaceData {
    time: u64,
    distance: u64,
}

impl Application {
    pub fn day6(self) {
        if self.args.part == 1 {
            self.d6p1();
        } else {
            self.d6p2();
        }
    }

    fn d6p1(self) {
        let races = read_data(&self.input);
        let mut answer = 1;
        for race in races {
            let mut wins = 0;
            for i in 0..race.time {
                if (race.time - i) * i > race.distance {
                    wins += 1;
                }
            }
            answer *= wins;
        }
        println!("{answer}");
    }

    fn d6p2(self) {
        let mut answer = 0;
        let race = reread_data(&self.input);
        for i in 0..race.time {
            if (race.time - i) * i > race.distance {
                answer += 1;
            }
        }
        println!("{answer}");
    }
}

fn read_data(input: &Vec<String>) -> Vec<RaceData> {
    let mut output = Vec::new();
    let times = input[0].split(":").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .into_iter()
        .map(|a| a.parse().expect("You effed up"))
        .collect::<Vec<u64>>();
    let distances = input[1].split(":").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .into_iter()
        .map(|a| a.parse().expect("You effed up"))
        .collect::<Vec<u64>>();
    for i in 0..times.len() {
        output.push(RaceData {
            time: times[i],
            distance: distances[i],
        });
    }
    return output;
}

fn reread_data(input: &Vec<String>) -> RaceData {
    let time = input[0].split(":").collect::<Vec<&str>>()[1]
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .expect("Somehow, it broke");
    let distance = input[1].split(":").collect::<Vec<&str>>()[1]
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .expect("Somehow, it also broke");
    return RaceData { time, distance };
}
