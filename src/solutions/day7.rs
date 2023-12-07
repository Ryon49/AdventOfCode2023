use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug)]
struct Hand {
    cards: String,
    t: u32,
    bid: u32,
}

fn card_to_value(c: char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 0, // change to lowest for part2, part1 = 11
        'T' => 10,
        _ => c.to_digit(10).unwrap(), // rest are digits
    }
}

impl Hand {
    fn new(cards: &str, bid: u32) -> Hand {
        let mut char_map: HashMap<char, u32> = HashMap::new();
        for c in cards.chars() {
            *char_map.entry(c).or_insert(0) += 1;
        }
        // after turn _input into char counts, turn it into a Vec ordered by count from high to low
        let mut char_count = char_map.into_iter().collect::<Vec<(char, u32)>>();
        char_count.sort_by(|(_, c1), (_, c2)| c2.cmp(c1));

        // figure out the Type of the cards, power of pattern match
        let t: u32 = match char_count[..] {
            [(_, 5)] => 7,                                 // FiveKind
            [(_, 4), (_, 1)] => 6,                         // FourKind
            [(_, 3), (_, 2)] => 5,                         // FullHouse
            [(_, 3), (_, 1), (_, 1)] => 4,                 // ThreeKind
            [(_, 2), (_, 2), (_, 1)] => 3,                 // TwoPair
            [(_, 2), (_, 1), (_, 1), (_, 1)] => 2,         // OnePair
            [(_, 1), (_, 1), (_, 1), (_, 1), (_, 1)] => 1, // HighCard
            _ => panic!("pattern exhausted"),
        };

        Hand {
            cards: cards.to_string(),
            t: t,
            bid: bid,
        }
    }

    // for part 2
    fn new2(cards: &str, bid: u32) -> Hand {
        let mut char_map: HashMap<char, u32> = HashMap::new();
        for c in cards.chars() {
            *char_map.entry(c).or_insert(0) += 1;
        }
        // after turn _input into char counts, turn it into a Vec ordered by count from high to low
        let mut char_count = char_map.into_iter().collect::<Vec<(char, u32)>>();
        char_count.sort_by(|(_, c1), (_, c2)| c2.cmp(c1));

        // remember the index where ('J', joker_count) is
        let (mut joker_count, mut joker_index) = (0, 0);
        for (i, (c, n)) in char_count.iter().enumerate() {
            if c == &'J' {
                joker_count = *n;
                joker_index = i;
                break;
            }
        }

        // remove 'J' first and add to the first in char_count
        if joker_count > 0 && joker_count != 5 {
            char_count.remove(joker_index);
            char_count[0] = (char_count[0].0, char_count[0].1 + joker_count);
        }

        // figure out the Type of the cards, power of pattern match
        let t: u32 = match char_count[..] {
            [(_, 5)] => 7,                                 // FiveKind
            [(_, 4), (_, 1)] => 6,                         // FourKind
            [(_, 3), (_, 2)] => 5,                         // FullHouse
            [(_, 3), (_, 1), (_, 1)] => 4,                 // ThreeKind
            [(_, 2), (_, 2), (_, 1)] => 3,                 // TwoPair
            [(_, 2), (_, 1), (_, 1), (_, 1)] => 2,         // OnePair
            [(_, 1), (_, 1), (_, 1), (_, 1), (_, 1)] => 1, // HighCard
            _ => panic!("pattern exhausted: {:?}\n", cards),
        };

        Hand {
            cards: cards.to_string(),
            t: t,
            bid: bid,
        }
    }

    fn compare(hand1: &Hand, hand2: &Hand) -> Ordering {
        hand1.t.cmp(&hand2.t).then_with(|| {
            for n in 0..5 {
                let c1 = hand1.cards.chars().nth(n).unwrap();
                let c2 = hand2.cards.chars().nth(n).unwrap();

                let v1 = card_to_value(c1);
                let v2 = card_to_value(c2);
                if v1 != v2 {
                    return v1.cmp(&v2);
                }
            }
            Ordering::Equal
        })
    }
}

#[allow(dead_code)]
pub fn part1(_input: &str) -> u32 {
    let mut hands = _input
        .split("\n")
        .map(|line| -> (&str, u32) {
            let items = line.split(" ").collect::<Vec<&str>>();
            return (items[0], items[1].parse::<u32>().unwrap());
        })
        .map(|(cards, bid)| Hand::new(cards, bid))
        .collect::<Vec<Hand>>();

    hands.sort_by(Hand::compare);

    hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| -> u32 { (i as u32 + 1) * hand.bid })
        .sum()

    //     hands.iter().for_each(|r| println!("{:?}", r));
    // 0
}

#[allow(dead_code)]
pub fn part2(_input: &str) -> u32 {
    let mut hands = _input
        .split("\n")
        .map(|line| -> (&str, u32) {
            let items = line.split(" ").collect::<Vec<&str>>();
            return (items[0], items[1].parse::<u32>().unwrap());
        })
        .map(|(cards, bid)| Hand::new2(cards, bid))
        .collect::<Vec<Hand>>();

    hands.sort_by(Hand::compare);

    hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| -> u32 { (i as u32 + 1) * hand.bid })
        .sum()

    //     hands.iter().for_each(|r| println!("{:?}", r));
    // 0
}
