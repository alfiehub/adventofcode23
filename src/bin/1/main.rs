fn main() {
    let input = include_str!("./input");
    let lines: Vec<&str> = input.split('\n').filter(|line| !line.is_empty()).collect();
    let simple_sum: u32 = lines
        .iter()
        .map(|line| {
            let numbers: Vec<_> = line.chars().filter(|c| c.is_numeric()).collect();
            match (
                numbers.first().and_then(|n| n.to_digit(10)),
                numbers.last().and_then(|n| n.to_digit(10)),
            ) {
                (Some(first), Some(last)) => first * 10 + last,
                _ => 0,
            }
        })
        .sum();
    println!("{simple_sum}");

    let tokens = &[
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let sum: usize = lines
        .into_iter()
        .map(|line| {
            let mut numbers = vec![];
            for i in 0..line.len() {
                tokens.iter().find(|(key, value)| {
                    if line[i..].starts_with(*key) {
                        numbers.push(*value);
                        true
                    } else {
                        false
                    }
                });
            }
            match (numbers.first(), numbers.last()) {
                (Some(first), Some(last)) => first * 10 + last,
                _ => 0,
            }
        })
        .sum();
    println!("{sum}");
}
