use std::io::Read;

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line|
        line.chars().map(|c| c.to_string().parse::<u8>().expect("parsing must work")).collect()
    ).collect()
}

fn maximum_joltage(batteries: &[u8]) -> u8 {
    let mut max_first_digit = batteries[0];
    let mut max_second_digit = batteries[1];

    for (index, value) in batteries.iter().enumerate().skip(1) {
        if (index < batteries.len() - 1) && *value > max_first_digit {
            max_first_digit = *value;
            max_second_digit = batteries[index+1];
        } else if *value > max_second_digit {
            max_second_digit = *value;
        }
    }

    10 * max_first_digit + max_second_digit
}

fn part1(batteries: &[Vec<u8>]) -> u32 {
    batteries.iter().map(|b| maximum_joltage(b) as u32).sum()
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("reading input must work");

    let batteries = parse_input(&input);

    println!("Part 1: {}", part1(&batteries));
}

#[test]
fn test_part_1() {
    assert_eq!(maximum_joltage(&[9,8,7,6,5,4,3,2,1,1,1,1,1,1,1]), 98);
    assert_eq!(maximum_joltage(&[8,1,1,1,1,1,1,1,1,1,1,1,1,1,9]), 89);
    assert_eq!(maximum_joltage(&[2,3,4,2,3,4,2,3,4,2,3,4,2,7,8]), 78);
    assert_eq!(maximum_joltage(&[8,1,8,1,8,1,9,1,1,1,1,2,1,1,1]), 92);
}