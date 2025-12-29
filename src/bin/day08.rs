use std::{
    collections::HashSet,
    io::{Read, stdin},
};

type Position = (u64, u64, u64);
type JunctionBox = usize;
type Connection = (JunctionBox, JunctionBox);

fn parse_input(input: &str) -> Vec<Position> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (x, rest) = line.split_once(',').expect("splitting must work");
            let (y, z) = rest.split_once(',').expect("splitting must work");

            (
                x.parse().expect("parsing must work"),
                y.parse().expect("parsing must work"),
                z.parse().expect("parsing must work"),
            )
        })
        .collect()
}

fn compute_distances(positions: &[Position]) -> Vec<(JunctionBox, JunctionBox, u64)> {
    let mut distances = vec![];
    for (jb1, pos1) in positions.iter().enumerate() {
        for (jb2, pos2) in positions.iter().enumerate().skip(jb1 + 1) {
            let distance = pos1.0.abs_diff(pos2.0).checked_pow(2).unwrap()
                + pos1.1.abs_diff(pos2.1).checked_pow(2).unwrap()
                + pos1.2.abs_diff(pos2.2).checked_pow(2).unwrap();

            distances.push((jb1, jb2, distance));
        }
    }

    distances.sort_unstable_by_key(|(_, _, d)| *d);
    distances
}

fn minimum_span_connections(distances: &[(JunctionBox, JunctionBox, u64)]) -> Vec<Connection> {
    let mut connections = Vec::new();
    let mut already_connected = HashSet::new();

    for (jb1, jb2, _) in distances {
        let jb1_not_connected = already_connected.insert(*jb1);
        let jb2_not_connected = already_connected.insert(*jb2);
        if jb1_not_connected || jb2_not_connected {
            connections.push((*jb1, *jb2));
        }
    }

    connections
}

fn create_circuits(connections: &[Connection]) -> Vec<usize> {
    let max_jb_id = connections.iter().fold(0, |acc, x| acc.max(x.0.max(x.1)));
    let mut circuit_idx = Vec::new();

    for i in 0..max_jb_id+1 {
        circuit_idx.push(i);
    }

    let mut circuit_idx_has_changed = true;

    while circuit_idx_has_changed {
        circuit_idx_has_changed = false;

        for c in connections {
            if circuit_idx[c.0] != circuit_idx[c.1] {
                circuit_idx_has_changed = true;
                circuit_idx[c.0] = circuit_idx[c.0].max(circuit_idx[c.1]);
                circuit_idx[c.1] = circuit_idx[c.0].max(circuit_idx[c.1]);
            }
        }
    }

    circuit_idx
}

fn part1(circuits: &[usize]) -> u64 {
    let mut circuit_size = Vec::new();

    for circuit_id in 0..=*circuits.iter().max().unwrap() {
        circuit_size.push(circuits.iter().filter(|x| **x == circuit_id).count() as u64);
    }

    circuit_size.sort_unstable();
    circuit_size.iter().rev().take(3).product()
}

fn main() {
    //let mut input = String::new();
    //stdin().read_to_string(&mut input).unwrap();

    let input = include_str!("../../inputs/day08.txt");

    let positions = parse_input(&input);
    let distances = compute_distances(&positions);
    let connections: Vec<_> = distances.iter().map(|(jb1, jb2, d)| (*jb1, *jb2)).take(1000).collect();
    let circuits = create_circuits(&connections);

    println!("Part 1: {}", part1(&circuits));
}

#[test]
fn test_part1() {
    let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    let positions = parse_input(&input);
    let distances = compute_distances(&positions);
    //let connections = minimum_span_connections(&distances);
    let connections: Vec<_> = distances.iter().map(|(jb1, jb2, d)| (*jb1, *jb2)).take(10).collect();
    let circuits = create_circuits(&connections);

    assert_eq!(part1(&circuits), 40);
}
