use std::io::{Read, stdin};

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

fn parse_input_part2(input: &str) -> (Vec<Vec<u64>>, Vec<char>) {
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
                operators.iter().map(|o| o[idx]).fold(1, |acc, v| acc * v)
            }
        })
        .sum()
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let (operators, operands) = parse_input(&input);

    println!("Part 1: {}", part1(&operators, &operands));
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
