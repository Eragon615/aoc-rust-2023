use crate::Application;
use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Clone)]
struct CardGame {
    hand: Hand,
    handtype: HandType,
    wager: u64,
}

#[derive(Debug, Clone)]
#[repr(u64)]
enum HandType {
    FiveOfAKind(Card) = 7,
    FourOfAKind(Card) = 6,
    FullHouse(Card, Card) = 5,
    ThreeOfAKind(Card) = 4,
    TwoPair(Card, Card) = 3,
    OnePair(Card) = 2,
    HighCard(Card) = 1,
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
            self.d7p2();
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

    fn d7p2(self) {}
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
        return HandType::FiveOfAKind(find_highcard(&handhash));
    }
    if handhash.len() == 2 {
        handhash.retain(|_, &mut n| n > 1);
        if handhash.len() == 1 {
            return HandType::FourOfAKind(find_highcard(&handhash));
        } else {
            let (x, y) = find_fullhouse_high(&handhash);
            return HandType::FullHouse(x, y);
        }
    }
    if handhash.len() == 3 {
        handhash.retain(|_, &mut n| n > 1);
        if handhash.len() == 1 {
            return HandType::ThreeOfAKind(find_highcard(&handhash));
        } else {
            let (x, y) = find_two_highcards(&handhash);
            return HandType::TwoPair(x, y);
        }
    }
    if handhash.len() == 4 {
        handhash.retain(|_, &mut n| n > 1);
        return HandType::OnePair(find_highcard(&handhash));
    }
    return HandType::HighCard(find_highcard(&handhash));
}

fn find_highcard(input: &HashMap<Card, u64>) -> Card {
    let mut output = Card::Two;
    for (card, _) in input {
        if *card as u64 > output as u64 {
            output = *card;
        }
    }
    return output;
}

fn find_two_highcards(input: &HashMap<Card, u64>) -> (Card, Card) {
    let mut keys = input.clone().into_keys();
    let out1 = keys.next().expect("Here's hoping");
    let out2 = keys.next().expect("This works");
    if out1 as u64 > out2 as u64 {
        return (out1, out2);
    } else {
        return (out2, out1);
    }
}

fn find_fullhouse_high(input: &HashMap<Card, u64>) -> (Card, Card) {
    let mut out3 = Card::Two;
    let mut out2 = Card::Two;
    for (k, v) in input {
        if *v == 3 {
            out3 = *k;
        } else {
            out2 = *k;
        }
    }
    return (out3, out2);
}

impl HandType {
    fn numeric_value(&self) -> u64 {
        match self {
            HandType::FiveOfAKind(_) => 7,
            HandType::FourOfAKind(_) => 6,
            HandType::FullHouse(_, _) => 5,
            HandType::ThreeOfAKind(_) => 4,
            HandType::TwoPair(_, _) => 3,
            HandType::OnePair(_) => 2,
            HandType::HighCard(_) => 1,
        }
    }
}

impl CardGame {
    fn compare_hands(&self, other: &CardGame) -> Ordering {
        if self.handtype.numeric_value() > other.handtype.numeric_value() {
            return Ordering::Greater;
        } else if self.handtype.numeric_value() < other.handtype.numeric_value() {
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
}
