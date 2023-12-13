#[derive(Clone, Debug)]
struct Sequence(Vec<isize>);

impl Sequence {
    fn next_sequence(&self) -> Option<Self> {
        if self.0.iter().all(|n| *n == 0) {
            None
        } else {
            Some(Self(self.0.windows(2).map(|w| w[1] - w[0]).collect()))
        }
    }

    fn expand(self) -> Vec<Self> {
        let mut expanded_sequences = vec![self];

        // Expand until we're done
        while let Some(next_sequence) = expanded_sequences.last().unwrap().next_sequence() {
            expanded_sequences.push(next_sequence);
        }

        expanded_sequences
    }
}

fn main() {
    let input: Vec<_> = include_str!("./input")
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect();

    let sequences: Vec<_> = input
        .into_iter()
        .map(|line| {
            Sequence(
                line.split(' ')
                    .map(|num| num.parse::<isize>().unwrap())
                    .collect(),
            )
        })
        .collect();

    // Part one
    let sum: isize = sequences
        .clone()
        .into_iter()
        .map(|sequence| {
            let expanded_sequences = sequence.expand();
            expanded_sequences
                .into_iter()
                .fold(0, |num, seq| num + seq.0.last().unwrap())
        })
        .sum();
    println!("{sum}");

    // Part two
    let sum: isize = sequences
        .into_iter()
        .map(|sequence| {
            let expanded_sequences = sequence.expand();
            expanded_sequences
                .into_iter()
                // This one has to be reversed because the order is important in subtraction.
                // Ie. (-2)-3 is not the same as 3-(-2)
                .rev()
                .fold(0, |num, seq| seq.0.first().unwrap() - num)
        })
        .sum();
    println!("{sum}");
}
