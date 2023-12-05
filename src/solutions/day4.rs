use regex::Regex;

#[derive(Debug)]
struct Card {
    id: usize,
    winning_number: Vec<u32>,
    player_number: Vec<u32>,
}

impl Card {
    fn winning_count(&self) -> u32 {
        let mut count = 0;
        for player in &self.player_number {
            if self.winning_number.contains(player) {
                count += 1;
            }
        }
        return count;
    }
    fn calculate_score(&self) -> u32 {
        let count = self.winning_count();
        if count == 0 {
            return 0;
        }
        return 2_u32.pow(self.winning_count() - 1);
    }
}

fn parse_cards(_input: &str) -> Vec<Card> {
    let re = Regex::new(r"Card +(?<id>\d+): +(?<winning>.+) \| +(?<player>.+)").unwrap();

    _input
        .split("\n")
        .into_iter()
        .map(|haystack| -> Card {
            let Some(caps) = re.captures(haystack) else { panic!("parse_cards: {}\n", haystack); };

            let (_, [id, winning, player]) = caps.extract();
            Card {
                id: id.parse::<usize>().unwrap(),
                winning_number: winning
                    .replace("  ", " ")
                    .split(" ")
                    .map(|v| v.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>(),
                player_number: player
                    .replace("  ", " ")
                    .split(" ")
                    .map(|v| v.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>(),
            }
        })
        .collect::<Vec<Card>>()
}

#[allow(dead_code)]
pub fn part1(_input: &str) -> u32 {
    let cards = parse_cards(_input);

    cards.iter().map(|card| card.calculate_score()).sum::<u32>()
}

#[allow(dead_code)]
pub fn part2(_input: &str) -> u32 {
    let cards = parse_cards(_input);

    // remembers the current copies of each card
    let mut card_counts: Vec<u32> = vec![1; cards.len() + 1];
    // because card id starts with 1, so card_counts[0] should be 0
    card_counts[0] = 0;

    for card in cards.iter() {
        let score = card.winning_count() as usize;
        for i in 1..score + 1 {
            let next_card = card.id + i;
            if next_card < cards.len() + 1 {
                card_counts[next_card] += card_counts[card.id]
            }
        }
    }

    for (i, count) in card_counts.iter().enumerate() {
        print!("{} = {}\n", i, count);
    }

    card_counts.iter().sum::<u32>()
}
