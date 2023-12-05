use rayon::prelude::*;

#[derive(Debug)]
struct Range {
    dst: usize,
    src: usize,
    range: usize,
}

impl Range {
    fn in_src_range(&self, src: usize) -> bool {
        src >= self.src && src < self.src + self.range
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    /// If custom mapping, do it, else return src as new dst
    fn get_dst(&self, src: usize) -> usize {
        for range in self.ranges.iter() {
            if range.in_src_range(src) {
                let step = src - range.src;
                return range.dst + step;
            };
        }
        src
    }
}

fn parse_map(lines: &mut Vec<&str>) -> Map {
    lines.pop();
    let mut ranges = vec![];
    while let Some(line) = lines.pop() {
        if line.is_empty() {
            break;
        } else {
            let mut numbers = line.split(' ').map(|n| n.parse::<usize>().unwrap());
            ranges.push(Range {
                dst: numbers.next().unwrap(),
                src: numbers.next().unwrap(),
                range: numbers.next().unwrap(),
            });
        }
    }
    Map { ranges }
}

fn parse_maps(lines: &mut Vec<&str>) -> Vec<Map> {
    let mut maps = vec![];
    while !lines.is_empty() {
        maps.push(parse_map(lines));
    }
    maps
}

fn main() {
    let input = include_str!("./input");
    let mut lines: Vec<_> = input.split('\n').collect();
    // Pop off extra newline at end of input
    lines.pop();

    // Reverse lines so we can .pop()
    lines.reverse();

    let seeds: Vec<usize> = {
        let (_, seeds) = lines.pop().unwrap().split_once(": ").unwrap();
        seeds
            .split(' ')
            .map(|n| n.parse::<usize>().unwrap())
            .collect()
    };
    println!("{seeds:?}");

    lines.pop();
    let maps = parse_maps(&mut lines);
    println!("{maps:#?}");

    // There shouldnt be any more input
    assert!(lines.is_empty());

    // Part one
    let min_location = seeds
        .iter()
        .map(|seed| maps.iter().fold(*seed, |src, map| map.get_dst(src)))
        .min();

    println!("min_location: {min_location:?}");

    // Part two
    let seed_ranges: Vec<std::ops::Range<usize>> = seeds
        .chunks(2)
        .map(|chunk| {
            // Chunk has exactly two elements
            let start = *chunk.first().unwrap();
            let step = *chunk.last().unwrap();
            start..(start + step)
        })
        .collect();

    let min_range_location = seed_ranges
        // Use .par_iter() from Rayon to parallelize
        .par_iter()
        .flat_map(|range| {
            range
                .clone()
                .map(|seed| maps.iter().fold(seed, |src, map| map.get_dst(src)))
                .collect::<Vec<_>>()
        })
        // Calling .min() on ParallelIterator reduces memory footprint vs. .collect() all locations
        // in a Vec and then doing .min()
        .min();

    println!("min_range_location: {min_range_location:?}");
}
