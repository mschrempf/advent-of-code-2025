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
        } else if line.is_empty() {
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

fn part2(ranges: &[RangeInclusive<u64>]) -> u64 {
    let mut ranges = Vec::from_iter(ranges.iter().map(|r| (*r.start(), *r.end())));
    ranges.sort_by_key(|r| r.0);

    let mut full_ranges = vec![ranges[0]];

    for r in ranges.iter().skip(1) {
        let last_range = full_ranges.last_mut().unwrap();

        if last_range.1 >= r.0 {
            last_range.1 = last_range.1.max(r.1);
        } else {
            full_ranges.push(*r);
        }
    }

    full_ranges
        .iter()
        .map(|r| r.1 - r.0 + 1)
        .sum()
}

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("reading input must work");
    let (ranges, ids) = parse_input(&input);

    println!("Part 1: {}", part1(&ranges, &ids));
    println!("Part 2: {}", part2(&ranges));
}
#[test]
fn test_part2() {
    assert_eq!(part2(&[3..=5, 10..=14, 16..=20, 12..=18]), 14);
    assert_eq!(part2(&[3..=5, 4..=4]), 3);
    assert_eq!(part2(&[4..=4, 3..=5]), 3);

    assert_eq!(part2(&[1..=1, 3..=3, 1..=3]), 3);
}
