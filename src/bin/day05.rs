use std::{io::Read, ops::RangeInclusive, vec};

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
    // we hold a whole view of all possible id ranges and tag them if these ranges are "fresh" or "not fresh". The first range spans the whole possible range and is considered "not fresh".
    let mut full_ranges = vec![((0, std::u64::MAX), false)];

    // then we iterate over all  the "fresh" ranges we should add to the whole view and check various conditions
    for r in ranges {
        let mut range_to_add = (*r.start(), *r.end());

        let mut something_has_changed = true;

        while something_has_changed && range_to_add.0 <= range_to_add.1 {
            something_has_changed = false;

            let nof_ranges = full_ranges.len();
            for idx in 0..nof_ranges {
                let range_to_test = full_ranges[idx].0;
                let is_fresh = full_ranges[idx].1;

                if range_to_test.0 <= range_to_add.0 && range_to_add.0 <= range_to_test.1 {
                    // (maybe partial) overlap between the range to add and the range we already store
                    something_has_changed = true;
                    if is_fresh {
                        // both ranges are fresh, so, we just shift the range to add
                        range_to_add.0 = range_to_test.1 + 1;
                    } else {
                        // the stored range is not fresh, so we have to split it up
                        full_ranges.push(((range_to_add.0 + 1, range_to_test.1), false));
                        full_ranges[idx].0.1 = range_to_add.0 - 1;
                    }

                    break;
                }

                if range_to_test.0 <= range_to_add.1 && range_to_add.1 <= range_to_test.1 {
                    // test range contains end of range to add
                    something_has_changed = true;
                    if is_fresh {
                        // just shift the range to add
                        range_to_add.1 = range_to_test.0 - 1;
                    } else {
                        // just shift the unfresh range
                        full_ranges[idx].0.0 = range_to_add.1 + 1;
                    }

                    break;
                }

                if range_to_add.0 <= range_to_test.0 && range_to_test.0 <= range_to_add.1 {
                    // range to add contains start of range to test
                    something_has_changed = true;
                    if is_fresh {
                        full_ranges[idx].0.0 = range_to_add.1 + 1;
                    } else {
                        full_ranges.remove(idx);
                    }
                    break;
                }

                if range_to_add.0 <= range_to_test.1 && range_to_test.1 <= range_to_add.1 {
                    // range to add contains end of range to test
                    if is_fresh {
                        full_ranges[idx].0.1 = range_to_add.0 - 1;
                    } else {
                        full_ranges.remove(idx);
                    }
                    break;
                }
            }
        }

        if range_to_add.0 <= range_to_add.1 {
            full_ranges.push((range_to_add, true));
        }

        // clean up full_ranges
        full_ranges.retain(|(range, _)| range.0 <= range.1);

        // sort by range
        full_ranges.sort_by(|a, b| a.0.0.cmp(&b.0.0));
    }

    full_ranges
        .iter()
        .filter_map(|(r, is_fresh)| if *is_fresh { Some(r.1 - r.0 + 1) } else { None })
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
