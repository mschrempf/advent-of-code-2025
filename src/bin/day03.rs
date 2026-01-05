use std::{io::Read, time::Instant};

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<u64>().expect("parsing must work"))
                .collect()
        })
        .collect()
}

fn maximum_joltage(batteries: &[u64], nof_batteries: usize) -> u64 {
    let mut max_digit_indices = [0; 12];

    // fill each index
    for digit_index in 0..nof_batteries {
        let start_index = if digit_index > 0 {
            max_digit_indices[digit_index - 1] + 1
        } else {
            0
        };

        max_digit_indices[digit_index] = start_index;

        for battery_index in start_index..batteries.len() - nof_batteries + digit_index + 1 {
            if batteries[battery_index] > batteries[max_digit_indices[digit_index]] {
                max_digit_indices[digit_index] = battery_index;
            }
        }
    }

    let mut result = 0;
    for &d in &max_digit_indices[..nof_batteries] {
        result = result * 10 + batteries[d];
    }

    result
}

fn part1(batteries: &[Vec<u64>]) -> u64 {
    batteries.iter().map(|b| maximum_joltage(b, 2)).sum()
}

fn part2(batteries: &[Vec<u64>]) -> u64 {
    batteries.iter().map(|b| maximum_joltage(b, 12)).sum()
}

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("reading input must work");

    let before_parse = Instant::now();
    let batteries = parse_input(&input);
    let parse_time = before_parse.elapsed();

    let before_part1 = Instant::now();
    println!("Part 1 {}", part1(&batteries));
    let part1_time = before_part1.elapsed();

    let before_part2 = Instant::now();
    println!("Part 2 {}", part2(&batteries));
    let part2_time = before_part2.elapsed();

    println!("==========================================");
    println!("Parsing: {} µs", parse_time.as_micros());
    println!("Part 1 : {} µs", part1_time.as_micros());
    println!("Part 2 : {} µs", part2_time.as_micros());
    println!(
        "Total  : {} µs",
        parse_time.as_micros() + part1_time.as_micros() + part2_time.as_micros()
    );
}

#[test]
fn test_part_1() {
    assert_eq!(
        maximum_joltage(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 2),
        98
    );
    assert_eq!(
        maximum_joltage(&[8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 2),
        89
    );
    assert_eq!(
        maximum_joltage(&[2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 2),
        78
    );
    assert_eq!(
        maximum_joltage(&[8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 2),
        92
    );
}

#[test]
fn test_part_2() {
    assert_eq!(
        maximum_joltage(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 12),
        987654321111
    );
    assert_eq!(
        maximum_joltage(&[8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 12),
        811111111119
    );
    assert_eq!(
        maximum_joltage(&[2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 12),
        434234234278
    );
    assert_eq!(
        maximum_joltage(&[8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 12),
        888911112111
    );
}
