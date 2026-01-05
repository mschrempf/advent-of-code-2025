use std::{
    collections::HashSet,
    io::{Read, stdin},
    time::Instant,
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

fn part2(splitter_positions: &[Vec<usize>]) -> u64 {
    if splitter_positions.is_empty() || splitter_positions[0].is_empty() {
        return 0;
    }

    // in contrast to part 1, we don't just store the beam positions but also how many beams there are per position
    let mut current_beam_positions = Vec::new();
    current_beam_positions.push((splitter_positions[0][0], 1));

    for splitters in splitter_positions {
        let mut new_beam_positions = Vec::new();

        for (beam, amount) in &current_beam_positions {
            if splitters.contains(beam) {
                new_beam_positions.push((beam - 1, *amount));
                new_beam_positions.push((beam + 1, *amount));
            } else {
                new_beam_positions.push((*beam, *amount));
            }
        }

        current_beam_positions.clear();
        new_beam_positions.sort_unstable_by_key(|(beam, _)| *beam);

        for (new_beam, new_amount) in new_beam_positions {
            if let Some((beam, amount)) = current_beam_positions.last_mut()
                && *beam == new_beam
            {
                *amount += new_amount;
            } else {
                current_beam_positions.push((new_beam, new_amount));
            }
        }
    }

    current_beam_positions
        .iter()
        .map(|(_, amount)| *amount)
        .sum()
}

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Reading input must work");

    let before_parse = Instant::now();
    let splitter_positions = parse_input(&input);
    let parse_time = before_parse.elapsed();

    let before_part1 = Instant::now();
    println!("Part 1: {}", part1(&splitter_positions));
    let part1_time = before_part1.elapsed();

    let before_part2 = Instant::now();
    println!("Part 2: {}", part2(&splitter_positions));
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
