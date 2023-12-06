#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn calculate_distance(&self, charge_time: usize) -> usize {
        let speed = charge_time;
        let remaining_time = self.time - charge_time;
        speed * remaining_time
    }

    /// Count the charge times that results in a win
    fn winning_charge_count(&self) -> usize {
        (0..self.time)
            .filter(|charge_time| self.calculate_distance(*charge_time) > self.distance)
            .count()
    }
}

fn parse_line<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Vec<usize> {
    lines
        .next()
        .and_then(|line| line.split(':').last())
        .map(|numbers_str| {
            numbers_str
                .split_whitespace()
                .flat_map(str::parse)
                .collect()
        })
        .unwrap()
}

fn main() {
    let mut lines = include_str!("./input").split('\n');

    let times = parse_line(&mut lines);
    let distances = parse_line(&mut lines);

    let races: Vec<Race> = distances
        .iter()
        .zip(times.iter())
        .map(|(&distance, &time)| Race { time, distance })
        .collect();
    println!("{races:?}");

    let winning_charge_product: usize = races.iter().map(Race::winning_charge_count).product();
    println!("Winning alternatives {winning_charge_product}");

    let combined_race = Race {
        // Messy conversion into String and re-parsing
        time: str::parse(&times.iter().map(usize::to_string).collect::<String>()).unwrap(),
        distance: str::parse(&distances.iter().map(usize::to_string).collect::<String>()).unwrap(),
    };
    println!("{combined_race:#?}");
    println!(
        "combined_race winning_charge_count: {}",
        combined_race.winning_charge_count()
    );
}
