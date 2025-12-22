use std::io::Read;

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| {
            let line = line.trim();
            let factor = if line.starts_with("L") { -1 } else { 1 };
            let line = line.trim_matches('L').trim_matches('R');
            let amount: i32 = line.parse().expect("cannot parse input");
            factor * amount
        })
        .collect()
}

fn dial(rotations: &[i32]) -> Vec<i32> {
    let mut values = vec![50];

    for &r in rotations {
        values.push(
            (values
                .last()
                .expect("at least one element exists in values")
                + r)
                .rem_euclid(100),
        );
    }

    values
}

fn part1(input: &[i32]) -> usize {
    dial(input).iter().filter(|&&n| n == 0).count()
}

fn part2(input: &[i32]) -> usize {
    let mut transformed_input = Vec::new();

    for &n in input {
        for _i in 0..n.abs() {
            transformed_input.push(n.signum());
        }
    }

    part1(&transformed_input)
}

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Reading input must work");
    let parsed_input = parse_input(&input);
    println!("Part 1: {}", part1(&parsed_input));
    println!("Part 2: {}", part2(&parsed_input));
}

#[test]
fn test_part1() {
    let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

    assert_eq!(part1(&parse_input(input)), 3);
}

#[test]
fn test_part2() {
    let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

    assert_eq!(part2(&parse_input(input)), 6);
}
