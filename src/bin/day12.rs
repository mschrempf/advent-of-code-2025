use std::{
    io::{Read, stdin},
    time::Instant,
};

struct TreeArea {
    width: u64,
    height: u64,
    presents: Vec<u64>,
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<TreeArea>) {
    let mut present_sizes = Vec::new();
    let mut tree_areas = Vec::new();

    for part in input.split("\n\n") {
        if part.contains('#') {
            // its a present
            present_sizes.push(part.chars().filter(|c| *c == '#').count() as u64);
        } else {
            // its the part with the tree areas
            for line in part.trim().lines() {
                let (size, presents) = line.split_once(':').unwrap();
                let (width, height) = size.split_once('x').unwrap();
                let width = width.parse().unwrap();
                let height = height.parse().unwrap();
                let presents = presents
                    .split_whitespace()
                    .map(|p| p.parse().unwrap())
                    .collect();
                tree_areas.push(TreeArea {
                    width,
                    height,
                    presents,
                });
            }
        }
    }

    (present_sizes, tree_areas)
}

fn part1(present_sizes: &[u64], tree_areas: &[TreeArea]) -> u64 {
    let mut result = 0;

    for tree_area in tree_areas {
        let total_present_size: u64 = tree_area
            .presents
            .iter()
            .enumerate()
            .map(|(idx, amount)| present_sizes[idx] * amount)
            .sum();

        if total_present_size <= tree_area.width * tree_area.height {
            result += 1;
        }
    }

    result
}

fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("reading input must work");

    let before_parsing = Instant::now();
    let (present_sizes, tree_areas) = parse_input(&input);
    let parse_time = before_parsing.elapsed();

    let before_part1 = Instant::now();
    println!("Part 1: {}", part1(&present_sizes, &tree_areas));
    let part1_time = before_part1.elapsed();

    println!("==========================================");
    println!("Parsing: {} µs", parse_time.as_micros());
    println!("Part 1 : {} µs", part1_time.as_micros());
    println!(
        "Total  : {} µs",
        parse_time.as_micros() + part1_time.as_micros()
    );
}
