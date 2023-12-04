use std::collections::HashMap;

#[derive(Debug)]
struct Card {
    index: usize,
    winning_numbers: HashMap<usize, bool>,
    numbers: Vec<usize>,
}

impl Card {
    fn winning_numbers(&self) -> usize {
        self.numbers
            .iter()
            .filter(|n| self.winning_numbers.contains_key(n))
            .count()
    }

    fn points(&self) -> usize {
        let winning_numbers = self.winning_numbers();
        if winning_numbers > 0 {
            2_usize.pow(winning_numbers as u32 - 1)
        } else {
            0
        }
    }
}

impl Card {
    fn new(index: usize, winning_numbers: Vec<usize>, numbers: Vec<usize>) -> Self {
        Self {
            index,
            winning_numbers: winning_numbers.into_iter().map(|n| (n, true)).collect(),
            numbers,
        }
    }
}

#[derive(Default)]
struct Game {
    known_cards: HashMap<usize, usize>,
}

impl Game {
    // Returns the total cards added by this card (including itself)
    fn resolve_card(&mut self, cards: &[Card], card: &Card) -> usize {
        if let Some(value) = self.known_cards.get(&card.index) {
            *value
        } else {
            let mut sum = 1;
            if let Some(card_copies) =
                cards.get((card.index + 1)..(card.index + 1 + card.winning_numbers()))
            {
                card_copies.iter().for_each(|card_copy| {
                    sum += self.resolve_card(cards, card_copy);
                });
            };
            self.known_cards.insert(card.index, sum);
            sum
        }
    }
}

fn main() {
    let input = include_str!("./input");
    println!("{input}");
    let lines: Vec<_> = input.split('\n').filter(|line| !line.is_empty()).collect();

    let cards: Vec<_> = lines
        .into_iter()
        .enumerate()
        .flat_map(|(index, line)| {
            line.split_once(':')
                .and_then(|(_, game)| game.split_once('|'))
                .map(|(winning_str, numbers_str)| {
                    Card::new(
                        index,
                        winning_str
                            .split_whitespace()
                            // Beware flat_mapping results will simply remove errors
                            .flat_map(str::parse::<usize>)
                            .collect(),
                        numbers_str
                            .split_whitespace()
                            .flat_map(str::parse::<usize>)
                            .collect(),
                    )
                })
        })
        .collect();
    println!("{cards:#?}");

    // Part one
    let card_points: Vec<usize> = cards.iter().map(Card::points).collect();
    println!("{card_points:?}");
    let sum: usize = card_points.iter().sum();
    println!("{sum}");

    // Part two
    let mut game = Game::default();
    let resolved_cards: Vec<_> = cards
        .iter()
        .map(|card| game.resolve_card(&cards, card))
        .collect();
    let sum: usize = resolved_cards.iter().sum();
    println!("{sum}");
}
