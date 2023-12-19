use crate::Application;
use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Clone)]
struct CardGame {
    hand: Hand,
    handtype: HandType,
    wager: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
#[repr(u64)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[repr(u64)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

type Hand = [Card; 5];

impl Application {
    pub fn day7(self) {
        let games = parse_hands(&self.input);
        if self.args.part == 1 {
            self.d7p1(games);
        } else {
            self.d7p2(games);
        }
    }

    fn d7p1(self, input: Vec<CardGame>) {
        let mut answer = 0;
        let mut input = input.to_owned();
        input.sort_by(|a, b| a.compare_hands(&b));
        for i in 0..input.len() {
            answer += input[i].wager * (i as u64 + 1);
        }
        println!("{answer}");
    }

    fn d7p2(self, input: Vec<CardGame>) {
        let mut answer = 0;
        let mut input = input
            .iter()
            .map(|g| g.clone().promote_hand())
            .collect::<Vec<CardGame>>();
        input.sort_by(|a, b| a.compare_hands_v2(&b));
        for i in 0..input.len() {
            answer += input[i].wager * (i as u64 + 1);
        }
        println!("{answer}");
    }
}

fn parse_hands(input: &Vec<String>) -> Vec<CardGame> {
    let mut output = Vec::new();
    for line in input {
        let wager = line.split_whitespace().collect::<Vec<&str>>()[1]
            .parse()
            .expect("messed up the wager");
        let hand = line.split_whitespace().collect::<Vec<&str>>()[0]
            .chars()
            .collect();
        let hand = cards_to_hand(hand);
        let handtype = detect_type(&hand);
        output.push(CardGame {
            hand,
            handtype,
            wager,
        });
    }
    return output;
}

fn cards_to_hand(input: Vec<char>) -> Hand {
    let mut hand = Vec::new();
    for i in 0..5 {
        let card = match input[i] {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!("How'd you do that?"),
        };
        hand.push(card);
    }
    return hand.try_into().expect("You got too many or too few cards");
}

fn detect_type(input: &Hand) -> HandType {
    let mut handhash: HashMap<Card, u64> = HashMap::new();
    for card in input {
        handhash
            .entry(*card)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    if handhash.len() == 1 {
        // This must be 5 of a kind
        return HandType::FiveOfAKind;
    }
    if handhash.len() == 2 {
        handhash.retain(|_, &mut n| n > 1);
        if handhash.len() == 1 {
            return HandType::FourOfAKind;
        } else {
            return HandType::FullHouse;
        }
    }
    if handhash.len() == 3 {
        handhash.retain(|_, &mut n| n > 1);
        if handhash.len() == 1 {
            return HandType::ThreeOfAKind;
        } else {
            return HandType::TwoPair;
        }
    }
    if handhash.len() == 4 {
        handhash.retain(|_, &mut n| n > 1);
        return HandType::OnePair;
    }
    return HandType::HighCard;
}

impl CardGame {
    fn compare_hands(&self, other: &CardGame) -> Ordering {
        if self.handtype as u64 > other.handtype as u64 {
            return Ordering::Greater;
        } else if (self.handtype as u64) < (other.handtype as u64) {
            return Ordering::Less;
        } else {
            for i in 0..self.hand.len() {
                if self.hand[i] as u64 > other.hand[i] as u64 {
                    return Ordering::Greater;
                } else if (self.hand[i] as u64) < other.hand[i] as u64 {
                    return Ordering::Less;
                }
            }
        }
        panic!("Somehow they're the same!");
    }

    fn compare_hands_v2(&self, other: &CardGame) -> Ordering {
        if self.handtype as u64 > other.handtype as u64 {
            return Ordering::Greater;
        } else if (self.handtype as u64) < (other.handtype as u64) {
            return Ordering::Less;
        } else {
            for i in 0..self.hand.len() {
                if self.hand[i].game2_val() > other.hand[i].game2_val() {
                    return Ordering::Greater;
                } else if self.hand[i].game2_val() < other.hand[i].game2_val() {
                    return Ordering::Less;
                }
            }
        }
        panic!("V2 messed up comparing");
    }

    fn promote_hand(self) -> Self {
        let mut input = self.to_owned();
        let jokers = self.find_jokers();
        if jokers == 0 {
            return input;
        }
        match input.handtype {
            HandType::FiveOfAKind => {}
            HandType::FourOfAKind => input.handtype = HandType::FiveOfAKind,
            HandType::FullHouse => input.handtype = HandType::FiveOfAKind,
            HandType::ThreeOfAKind => input.handtype = HandType::FourOfAKind,
            HandType::TwoPair => {
                if jokers == 1 {
                    input.handtype = HandType::FullHouse;
                } else if jokers == 2 {
                    input.handtype = HandType::FourOfAKind;
                }
            }
            HandType::OnePair => input.handtype = HandType::ThreeOfAKind,
            HandType::HighCard => input.handtype = HandType::OnePair,
        }
        return input;
    }

    fn find_jokers(&self) -> u64 {
        let mut output = 0;
        for card in self.hand {
            if card == Card::Jack {
                output += 1;
            }
        }
        return output;
    }
}

impl Card {
    fn game2_val(&self) -> u64 {
        if *self == Card::Jack {
            return 1;
        } else {
            return *self as u64;
        }
    }
}
