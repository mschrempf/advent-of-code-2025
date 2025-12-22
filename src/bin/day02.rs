use std::{io::Read, ops::Rem};

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .split(',')
        .map(|part| {
            let (start, end) = part.split_once('-').expect("parsing error");

            (
                start.parse().expect("must be an integer"),
                end.parse().expect("must be an integer"),
            )
        })
        .collect()
}

fn nof_digits(mut id: u64) -> u32 {
    let mut result = 0;

    while id > 0 {
        result += 1;
        id /= 10;
    }

    result
}

fn is_invalid(id: u64, max_pattern_length: u32) -> bool {
    let nof_digits = nof_digits(id);

    for pattern_length in 1..=max_pattern_length {
        let repetitions = nof_digits / pattern_length;

        if repetitions * pattern_length != nof_digits {
            continue;
        }

        let pattern = id.rem(10_u64.pow(pattern_length));
        let mut number = 0;

        for _i in 0..repetitions {
            number = (number * 10_u64.pow(pattern_length)) + pattern;
        }

        if number == id {
            return true;
        }
    }

    false
}

fn sum_of_invalid_ids_part1(start: u64, end: u64) -> u64 {
    let mut sum = 0;

    let start_nof_digits = nof_digits(start);

    let nof_digits_to_shift = (start_nof_digits / 2) + start_nof_digits.rem(2);

    let mut prefix = start / 10_u64.pow(nof_digits_to_shift);

    while (prefix * 10_u64.pow(nof_digits(prefix)) + prefix) <= end {
        if (prefix * 10_u64.pow(nof_digits(prefix)) + prefix) >= start {
            sum += prefix * 10_u64.pow(nof_digits(prefix)) + prefix;
        }
        prefix += 1;
    }

    sum
}

fn sum_of_invalid_ids_part2(start: u64, end: u64) -> u64 {
    let mut sum = 0;

    let start_nof_digits = nof_digits(start);
    let end_nof_digits = nof_digits(end);

    for pattern_length in 1..=(start_nof_digits.max(end_nof_digits) / 2) {
        if start_nof_digits.rem(pattern_length) != 0 && end_nof_digits.rem(pattern_length) != 0 {
            // we cannot create a valid pattern with this length
            continue;
        }

        let repetitions = start_nof_digits / pattern_length;

        let mut pattern = (start / 10_u64.pow(start_nof_digits - pattern_length))
            .min(end / 10_u64.pow(end_nof_digits - pattern_length));

        while nof_digits(pattern) == pattern_length {
            let mut number = 0;
            for _i in 0..repetitions {
                number = (number * 10_u64.pow(pattern_length)) + pattern;
            }

            if number >= start && number <= end && !is_invalid(number, pattern_length - 1) {
                sum += number;
            }

            if number > end {
                break;
            }

            while nof_digits(number) < end_nof_digits {
                // try more repetitions
                number = (number * 10_u64.pow(pattern_length)) + pattern;
                if number >= start && number <= end && !is_invalid(number, pattern_length - 1) {
                    sum += number;
                }
            }

            pattern += 1;
        }
    }

    sum
}

fn part1(ranges: &[(u64, u64)]) -> u64 {
    ranges
        .iter()
        .map(|(start, end)| sum_of_invalid_ids_part1(*start, *end))
        .sum()
}

fn part2(ranges: &[(u64, u64)]) -> u64 {
    ranges
        .iter()
        .map(|(start, end)| sum_of_invalid_ids_part2(*start, *end))
        .sum()
}

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Reading input did not work");
    let ranges = parse_input(&input);
    println!("Part 1 {}", part1(&ranges));
    println!("Part 2 {}", part2(&ranges));
}

#[test]
fn test_part1() {
    assert_eq!(sum_of_invalid_ids_part1(11, 22), 11 + 22);
    assert_eq!(sum_of_invalid_ids_part1(95, 115), 99);
    assert_eq!(sum_of_invalid_ids_part1(998, 1012), 1010);
    assert_eq!(sum_of_invalid_ids_part1(1188511880, 1188511890), 1188511885);
    assert_eq!(sum_of_invalid_ids_part1(222220, 222224), 222222);
    assert_eq!(sum_of_invalid_ids_part1(1698522, 1698528), 0);
    assert_eq!(sum_of_invalid_ids_part1(446443, 446449), 446446);
    assert_eq!(sum_of_invalid_ids_part1(38593856, 38593862), 38593859);

    let input: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    let ranges = parse_input(&input);

    assert_eq!(part1(&ranges), 1227775554);
}

#[test]
fn test_part2() {
    assert_eq!(sum_of_invalid_ids_part2(11, 22), 11 + 22);
    assert_eq!(sum_of_invalid_ids_part2(95, 115), 99 + 111);
    assert_eq!(sum_of_invalid_ids_part2(998, 1012), 999 + 1010);
    assert_eq!(sum_of_invalid_ids_part2(1188511880, 1188511890), 1188511885);
    assert_eq!(sum_of_invalid_ids_part2(222220, 222224), 222222);
    assert_eq!(sum_of_invalid_ids_part2(1698522, 1698528), 0);
    assert_eq!(sum_of_invalid_ids_part2(446443, 446449), 446446);
    assert_eq!(sum_of_invalid_ids_part2(38593856, 38593862), 38593859);
    assert_eq!(sum_of_invalid_ids_part2(565653, 565659), 565656);
    assert_eq!(sum_of_invalid_ids_part2(824824821, 824824827), 824824824);
    assert_eq!(sum_of_invalid_ids_part2(2121212118, 2121212124), 2121212121);

    let input: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let ranges = parse_input(&input);
    assert_eq!(part2(&ranges), 4174379265);
}
