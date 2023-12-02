use std::str::FromStr;

#[derive(Debug)]
struct Round {
    red: usize,
    green: usize,
    blue: usize,
}

impl Round {
    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

fn parse_observation(value: &str) -> Result<usize, <usize as FromStr>::Err> {
    value.split_whitespace().next().unwrap().parse::<usize>()
}

fn line_to_observation<'a>(color: &str, mut line: impl Iterator<Item = &'a &'a str>) -> usize {
    line.find(|line| line.ends_with(color))
        .and_then(|line| parse_observation(line).ok())
        .unwrap_or_default()
}

impl From<&str> for Round {
    fn from(value: &str) -> Self {
        let split: Vec<_> = value
            .split(',')
            .collect();
        let red = line_to_observation("red", split.iter());
        let blue = line_to_observation("blue", split.iter());
        let green = line_to_observation("green", split.iter());
        Self { red, green, blue }
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<Round>,
}

impl Game {
    /// If any of the rounds exceeds the maximum allowed value, the game is not possible
    fn is_possible(&self) -> bool {
        !self
            .rounds
            .iter()
            .any(|round| round.red > 12 || round.green > 13 || round.blue > 14)
    }

    /// The minimum amount of cubes required to play this game
    fn minimum_round(&self) -> Round {
        Round {
            red: self.rounds.iter().map(|round| round.red).max().unwrap(),
            green: self.rounds.iter().map(|round| round.green).max().unwrap(),
            blue: self.rounds.iter().map(|round| round.blue).max().unwrap(),
        }
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let (game, rounds) = value.split_once(':').unwrap();
        let id = game
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        Self {
            id,
            rounds: rounds.split(';').map(|round| round.into()).collect(),
        }
    }
}

fn main() {
    let test = include_str!("./input");
    let lines: Vec<_> = test.split('\n').filter(|line| !line.is_empty()).collect();

    let games: Vec<_> = lines.into_iter().map(Game::from).collect();
    println!("{games:#?}");

    let possible_games: Vec<_> = games.iter().filter(|game| game.is_possible()).collect();
    println!("{possible_games:#?}");

    let sum: usize = possible_games.into_iter().map(|game| game.id).sum();
    println!("{sum}");

    // Part two
    let powers_sum: usize = games
        .iter()
        .map(Game::minimum_round)
        .map(|round| round.power())
        .sum();
    println!("{powers_sum}");
}
