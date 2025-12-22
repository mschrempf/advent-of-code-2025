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

fn _is_invalid(id: &str) -> bool {
    let len = id.len();

    (len.rem_euclid(2) == 0) && (id[0..len / 2] == id[len / 2..])
}

fn sum_of_invalid_ids(start: u64, end: u64) -> u64 {
    let mut sum = 0;

    let start_nof_digits = start.to_string().len() as u64;
    //let end_nof_digits = end.to_string().len() as u64;

    let nof_digits_to_shift = ((start_nof_digits / 2) + start_nof_digits.rem(2)) as u32;

    let mut prefix = start / 10_u64.pow(nof_digits_to_shift);

    while (prefix * 10_u64.pow(prefix.to_string().len() as u32) + prefix) <= end {
        if (prefix * 10_u64.pow(prefix.to_string().len() as u32) + prefix) >= start {
            sum += prefix * 10_u64.pow(prefix.to_string().len() as u32) + prefix;
        }
        prefix += 1;
    }

    sum
}

fn part1(input: &[(u64, u64)]) -> u64 {
    let mut result = 0;
    for (start, end) in input {
        result += sum_of_invalid_ids(*start, *end);
    }
    result
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("Reading input did not work");
    let ranges = parse_input(&input);
    println!("Part 1 {}", part1(&ranges));
}

#[test]
fn test_part1() {
    assert_eq!(sum_of_invalid_ids(11, 22), 11 + 22);
    assert_eq!(sum_of_invalid_ids(95, 115), 99);
    assert_eq!(sum_of_invalid_ids(998, 1012), 1010);
    assert_eq!(sum_of_invalid_ids(1188511880, 1188511890), 1188511885);
    assert_eq!(sum_of_invalid_ids(222220, 222224), 222222);
    assert_eq!(sum_of_invalid_ids(1698522, 1698528), 0);
    assert_eq!(sum_of_invalid_ids(446443, 446449), 446446);
    assert_eq!(sum_of_invalid_ids(38593856, 38593862), 38593859);

    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    let ranges = parse_input(&input);

    assert_eq!(part1(&ranges), 1227775554);
}
