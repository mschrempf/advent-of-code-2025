use std::{
    io::{Read, stdin},
    time::Instant,
};

fn parse_input(input: &str) -> (Vec<Vec<u64>>, Vec<char>) {
    let mut operators = vec![];
    let mut operands = vec![];

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        if line.contains("+") {
            for c in line.chars() {
                if c.is_ascii_punctuation() {
                    operands.push(c);
                }
            }
        } else {
            operators.push(
                line.split_whitespace()
                    .map(|part| part.parse().unwrap())
                    .collect(),
            );
        }
    }

    (operators, operands)
}

fn part1(operators: &[Vec<u64>], operands: &[char]) -> u64 {
    operands
        .iter()
        .enumerate()
        .map(|(idx, operand)| {
            if operand == &'+' {
                operators.iter().map(|o| o[idx]).sum::<u64>()
            } else {
                operators.iter().map(|o| o[idx]).product::<u64>()
            }
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    let mut input: Vec<_> = input
        .lines()
        .filter_map(|l| {
            if l.is_empty() {
                None
            } else {
                Some(l.as_bytes())
            }
        })
        .collect();
    let operator_line = input.pop().unwrap();

    let mut result = 0;
    let mut current_operands = vec![0];
    let mut current_operator = b'+';

    for (idx, c) in operator_line.iter().enumerate() {
        if !c.is_ascii_whitespace() {
            current_operands.pop();

            if current_operator == b'+' {
                result += current_operands.iter().sum::<u64>();
            } else {
                result += current_operands.iter().product::<u64>();
            }

            current_operator = *c;
            current_operands.clear();
        }

        current_operands.push(input.iter().map(|line| line[idx]).fold(0, |acc, c| {
            if c.is_ascii_whitespace() {
                acc
            } else {
                acc * 10 + (c - b'0') as u64
            }
        }));
    }

    if current_operator == b'+' {
        result += current_operands.iter().sum::<u64>();
    } else {
        result += current_operands.iter().product::<u64>();
    }

    result
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let before_parse = Instant::now();
    let (operators, operands) = parse_input(&input);
    let parse_time = before_parse.elapsed();

    let before_part1 = Instant::now();
    println!("Part 1: {}", part1(&operators, &operands));
    let part1_time = before_part1.elapsed();

    let before_part2 = Instant::now();
    println!("Part 2: {}", part2(&input));
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
fn test_part1() {
    let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";
    let (operators, operands) = parse_input(&input);

    assert_eq!(part1(&operators, &operands), 4277556);
}

#[test]
fn test_part2() {
    let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

    assert_eq!(part2(&input), 3263827);
}
