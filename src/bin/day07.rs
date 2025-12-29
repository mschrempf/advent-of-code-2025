use std::{
    collections::HashSet,
    io::{Read, stdin},
};

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .filter(|line| line.contains('^'))
        .map(|line| {
            line.char_indices()
                .filter_map(|(idx, c)| (c == '^').then_some(idx))
                .collect()
        })
        .collect()
}

fn part1(splitter_positions: &[Vec<usize>]) -> u64 {
    if splitter_positions.is_empty() || splitter_positions[0].is_empty() {
        return 0;
    }

    let mut number_of_splits = 0;

    let mut current_beam_positions = HashSet::new();
    current_beam_positions.insert(splitter_positions[0][0]);

    for splitters in splitter_positions {
        let mut new_beam_positions = HashSet::new();

        for beam in &current_beam_positions {
            if splitters.contains(beam) {
                number_of_splits += 1;
                new_beam_positions.insert(beam - 1);
                new_beam_positions.insert(beam + 1);
            } else {
                new_beam_positions.insert(*beam);
            }
        }

        current_beam_positions = new_beam_positions;
    }

    number_of_splits
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let splitter_positions = parse_input(&input);

    println!("Part 1: {}", part1(&splitter_positions));
}
