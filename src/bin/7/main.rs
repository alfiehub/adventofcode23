use std::{
    cmp::Ordering,
    collections::{hash_map::Entry, HashMap},
};

#[derive(Clone, Debug)]
struct Card {
    face: char,
    /// Jokers are imposters with another face
    joker: bool,
}
impl Card {
    fn to_value(&self, face_value: bool) -> usize {
        if self.joker && !face_value {
            // Jokers are the weakest and we ignore their fake face
            1
        } else {
            match self.face {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                '2'..='9' => self.face.to_digit(10).unwrap() as usize,
                _ => panic!(),
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Hand {
    cards: [Card; 5],
}

impl Hand {
    fn new(line: &str) -> (Self, usize) {
        let (cards, bid) = line.split_once(' ').unwrap();
        (
            Self {
                cards: cards
                    .chars()
                    .map(|value| Card {
                        face: value,
                        joker: false,
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            },
            bid.parse().unwrap(),
        )
    }

    /// Find the best hand that we can replace the joker with
    fn resolve_joker(self) -> Self {
        let joker_positions: Vec<_> = self
            .cards
            .iter()
            .enumerate()
            .filter_map(|(i, card)| if card.face == 'J' { Some(i) } else { None })
            .collect();
        if joker_positions.is_empty() {
            self
        } else {
            // The alternatives that we can replace the joker with
            let mut replacements: Vec<_> = self
                .cards
                .iter()
                .filter(|card| card.face != 'J')
                .cloned()
                .collect();

            // Mark all replacment cards as jokers
            replacements.iter_mut().for_each(|card| card.joker = true);

            // What if the joker can be 'A'. Is redundant if replacements already contains 'A'
            replacements.push(Card {
                face: 'A',
                joker: true,
            });

            // Generate all hands with the joker replaced
            // For each joker_position, replace it with all possibilities
            let mut hands: Vec<Hand> =
                joker_positions
                    .into_iter()
                    .fold(vec![self], |hands, joker_position| {
                        hands
                            .into_iter()
                            .flat_map(|hand| {
                                replacements
                                    .iter()
                                    .map(|card| {
                                        let mut new_hand = hand.clone();
                                        new_hand.cards[joker_position] = card.clone();
                                        new_hand
                                    })
                                    .collect::<Vec<_>>()
                            })
                            .collect()
                    });

            // Sort and return the best hand
            hands.sort();
            hands.last().unwrap().clone()
        }
    }

    /// Get the rank of a hand
    /// five of a kind => 6
    /// four of a kind => 5
    /// full house => 4
    /// three of a kind => 3
    /// two pair => 2
    /// one pair => 1
    /// high card => 0
    fn hand_rank(&self) -> usize {
        let frequencies: HashMap<usize, usize> =
            self.cards.iter().fold(HashMap::new(), |mut acc, card| {
                match acc.entry(card.to_value(true)) {
                    Entry::Occupied(mut o) => {
                        *o.get_mut() += 1;
                    }
                    Entry::Vacant(v) => {
                        v.insert(1);
                    }
                };
                acc
            });
        let frequencies: Vec<_> = frequencies.values().cloned().collect();
        let max_of_a_kind = *frequencies.iter().max().unwrap();
        if max_of_a_kind == 1 {
            // High card
            0
        } else if max_of_a_kind == 2 {
            // Either one or two pairs
            frequencies.iter().filter(|n| **n == 2).count()
        } else if max_of_a_kind == 3 {
            // Do we have a house?
            if frequencies.iter().any(|v| *v == 2) {
                4
            } else {
                3
            }
        } else {
            max_of_a_kind + 1
        }
    }
}

impl std::cmp::Eq for Hand {}
impl std::cmp::PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        match self.cmp(other) {
            Ordering::Equal => true,
            Ordering::Greater | Ordering::Less => false,
        }
    }
}
impl std::cmp::PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let ordering = self.hand_rank().cmp(&other.hand_rank());
        match ordering {
            Ordering::Equal => {
                // If hand_rank is equal, then we need to compare each card
                let zipped_cards = self.cards.iter().zip(other.cards.iter());
                for next_pair in zipped_cards {
                    // When comparing each card, need to treat the joker as 1, so face_value ==
                    // false
                    let card_ordering = next_pair
                        .0
                        .to_value(false)
                        .cmp(&next_pair.1.to_value(false));
                    match card_ordering {
                        Ordering::Equal => continue,
                        Ordering::Greater | Ordering::Less => return card_ordering,
                    }
                }
                // If all cards are equal then we are truly equal
                Ordering::Equal
            }
            Ordering::Greater | Ordering::Less => ordering,
        }
    }
}

fn main() {
    let lines = include_str!("./input")
        .split('\n')
        .filter(|line| !line.is_empty());

    // Put hand in a tuple alongside bid (so we can keep track of the corresponding bid to a hand)
    let mut hands: Vec<(Hand, usize)> = lines.map(Hand::new).collect();

    // Simply sorting the tuple seems to work as the input doesnt have two equal hands with different
    // bids, lmao
    hands.sort();

    let sum: usize = hands
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid)
        .sum();
    println!("{sum}");

    // Part two
    let mut hands_joker_resolved: Vec<_> = hands
        .into_iter()
        .map(|(hand, bid)| (hand.resolve_joker(), bid))
        .collect();
    hands_joker_resolved.sort();

    let sum: usize = hands_joker_resolved
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid)
        .sum();
    println!("{sum}");
}
