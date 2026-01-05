use std::{
    collections::HashSet,
    io::{Read, stdin},
};

struct Machine {
    target_lights: u64,
    buttons: Vec<u64>,
    joltage_levels: Vec<u64>,
}

impl Machine {
    fn from_str(input: &str) -> Self {
        let mut target_lights = 0;
        let mut buttons = Vec::new();
        let mut joltage_levels = Vec::new();

        for p in input.split_whitespace() {
            if p.starts_with('[') {
                target_lights = build_target_lights(p);
            } else if p.starts_with('(') {
                buttons.push(build_button(p));
            } else if p.starts_with('{') {
                joltage_levels = build_joltage_levels(p);
            } else {
                panic!("machine building error");
            }
        }

        Self {
            target_lights,
            buttons,
            joltage_levels,
        }
    }
}

fn build_target_lights(s: &str) -> u64 {
    let s = s.trim_start_matches('[').trim_end_matches(']');
    let mut target_lights = 0;
    for (idx, b) in s.char_indices() {
        if b == '#' {
            target_lights |= 1 << idx;
        }
    }
    target_lights
}

fn build_button(s: &str) -> u64 {
    let s = s.trim_start_matches('(').trim_end_matches(')');
    let mut button = 0;
    for bit in s.split(',') {
        let bit: u64 = bit.parse().unwrap();
        button |= 1 << bit;
    }
    button
}

fn build_joltage_levels(s: &str) -> Vec<u64> {
    let s = s.trim_start_matches('{').trim_end_matches('}');

    s.split(',')
        .map(|joltage_level| joltage_level.parse().unwrap())
        .collect()
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(Machine::from_str)
        .collect()
}

fn min_number_of_button_presses(m: &Machine) -> usize {
    let mut nof_presses = 0;

    let mut open_states = HashSet::new();
    open_states.insert(0);

    loop {
        nof_presses += 1;
        let mut new_open_states = HashSet::new();

        for open_state in open_states {
            for b in &m.buttons {
                let s = open_state ^ b;
                if s == m.target_lights {
                    return nof_presses;
                }
                new_open_states.insert(s);
            }
        }

        open_states = new_open_states;
    }
}

fn part1(machines: &[Machine]) -> usize {
    machines.iter().map(min_number_of_button_presses).sum()
}

fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("reading input must work");
    let machines = parse_input(&input);

    println!("Part 1: {}", part1(&machines));
}

#[test]
fn test_part1() {
    assert_eq!(
        min_number_of_button_presses(&Machine::from_str(
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"
        )),
        2
    );

    assert_eq!(
        min_number_of_button_presses(&Machine::from_str(
            "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}"
        )),
        3
    );

    assert_eq!(
        min_number_of_button_presses(&Machine::from_str(
            "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"
        )),
        2
    );

    assert_eq!(
        min_number_of_button_presses(&Machine::from_str(
            " [.#...#.#..] (3,5) (1,2,3,4,6,8,9) (0,4,8,9) (2,3,4,5,6,7,8,9) (6,8) (1,4,5,7,8) (1,2,5,9) (0,1,3,4,5,6,9) {5,22,17,25,34,32,38,24,45,22}"
        )),
        2
    );

    let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";
    let machines = parse_input(input);
    assert_eq!(part1(&machines), 7);
}
