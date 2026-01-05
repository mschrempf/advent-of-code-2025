use std::{
    io::{Read, stdin},
    time::Instant,
};

type Tile = (u64, u64);

fn parse_input(input: &str) -> Vec<Tile> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (x, y) = line.split_once(',').expect("input parsing must work");

            (
                x.parse().expect("input parsing must work"),
                y.parse().expect("input parsing must work"),
            )
        })
        .collect()
}

fn rectangle_area(tile1: Tile, tile2: Tile) -> u64 {
    (tile1.0.abs_diff(tile2.0) + 1) * (tile1.1.abs_diff(tile2.1) + 1)
}

fn part1(tiles: &[Tile]) -> u64 {
    let mut max_area = 0;

    for (idx, t1) in tiles.iter().enumerate() {
        for t2 in tiles.iter().skip(idx + 1) {
            max_area = max_area.max(rectangle_area(*t1, *t2));
        }
    }

    max_area
}

fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("reading input must work");

    let before_parse = Instant::now();
    let tiles = parse_input(&input);
    let parse_time = before_parse.elapsed();

    let before_part1 = Instant::now();
    println!("Part 1: {}", part1(&tiles));
    let part1_time = before_part1.elapsed();

    let before_part2 = Instant::now();
    let part2_time = before_part2.elapsed();

    println!("==========================================");
    println!("Parsing: {} µs", parse_time.as_micros());
    println!("Part 1 : {} µs", part1_time.as_micros());
    // println!("Part 2 : {} µs", part2_time.as_micros());
    println!(
        "Total  : {} µs",
        parse_time.as_micros() + part1_time.as_micros() + part2_time.as_micros()
    );
}

#[test]
fn test_part1() {
    let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";
    let tiles = parse_input(input);
    assert_eq!(part1(&tiles), 50);
}
