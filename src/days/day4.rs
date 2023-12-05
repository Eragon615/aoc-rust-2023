use crate::Application;

#[derive(Debug)]
struct Cards {
    cards: Vec<Card>,
}

#[derive(Debug)]
struct Card {
    id: u64,
    winning_nums: Vec<u64>,
    played_nums: Vec<u64>,
}

impl Application {
    pub fn day4(self) {
        let cards = generate_cards(&self.input);
        if self.args.part == 1 {
            self.d4p1(cards);
        } else {
            self.d4p2();
        }
    }

    fn d4p1(self, cards: Cards) {
        let mut answer = 0;
        for c in cards.cards {
            let mut matches = 0;
            for winning_num in &c.winning_nums {
                for played_num in &c.played_nums {
                    if played_num == winning_num {
                        matches += 1;
                        continue;
                    }
                }
            }
            if matches == 0 {
                continue;
            } else if matches == 1 {
                answer += 1;
            } else {
                answer += u64::pow(2, matches - 1);
            }
        }
        println!("{answer}");
    }

    fn d4p2(self) {}
}

fn generate_cards(input: &Vec<String>) -> Cards {
    let mut cards: Vec<Card> = Vec::new();
    for line in input {
        let line: Vec<&str> = line.split(':').collect();
        let id: u64 = line[0].split_whitespace().collect::<Vec<&str>>()[1]
            .parse()
            .expect("Something broke");
        let winning_nums: Vec<u64> = line[1].split('|').collect::<Vec<&str>>()[0]
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|n| n.parse::<u64>().expect("Oops I broke it"))
            .collect::<Vec<u64>>();
        let played_nums: Vec<u64> = line[1].split('|').collect::<Vec<&str>>()[1]
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|n| n.parse::<u64>().expect("Oops I broke it"))
            .collect::<Vec<u64>>();
        cards.push(Card {
            id,
            winning_nums,
            played_nums,
        })
    }
    return Cards { cards };
}
