use std::{io::Read, ops::RangeInclusive};

fn parse_input(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let mut ranges = Vec::new();
    let mut ids = Vec::new();

    for line in input.lines() {
        if line.contains("-") {
            let (start, end) = line.split_once("-").expect("parsing must work");

            let start = start.parse().expect("parsing must work");
            let end = end.parse().expect("parsing must work");

            ranges.push(start..=end);
        } else         if line.is_empty() {
            continue;
        } else {
            ids.push(line.parse().expect("parsing must work"));
        }
    }

    (ranges, ids)
}

fn is_fresh(id: u64, ranges: &[RangeInclusive<u64>]) -> bool {
    ranges.iter().any(|r| r.contains(&id))
}

fn part1(ranges: &[RangeInclusive<u64>], ids: &[u64]) -> usize {
    ids.iter().filter(|id| is_fresh(**id, ranges)).count()
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("reading input must work");
    let (ranges, ids) = parse_input(&input);

    println!("Part 1: {}", part1(&ranges, &ids));
}