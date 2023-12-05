use crate::Application;

#[derive(Debug)]
struct Cards {
    cards: Vec<Card>,
}

#[derive(Debug)]
struct Card {
    _id: u64,
    winning_nums: Vec<u64>,
    played_nums: Vec<u64>,
    copies: usize,
    wins: usize,
}

impl Application {
    pub fn day4(self) {
        let cards = generate_cards(&self.input);
        if self.args.part == 1 {
            self.d4p1(cards);
        } else {
            self.d4p2(cards);
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

    fn d4p2(self, cards: Cards) {
        let mut cards: Vec<Card> = cards.cards.into();
        let mut answer = 0;
        for i in 0..cards.len() {
            let mut wins = 0;
            for winning_num in &cards[i].winning_nums {
                for played_num in &cards[i].played_nums {
                    if played_num == winning_num {
                        wins += 1;
                    }
                }
            }
            cards[i].wins = wins;
            for j in (i + 1)..(i + cards[i].wins + 1) {
                cards[j].copies += 1 * cards[i].copies;
            }
            answer += cards[i].copies;
        }
        println!("{answer}")
    }
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
            _id: id,
            winning_nums,
            played_nums,
            copies: 1,
            wins: 0,
        })
    }
    return Cards { cards };
}
