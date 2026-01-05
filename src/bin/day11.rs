use std::{collections::HashMap, io::{Read, stdin}};

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    let mut output: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines().filter(|l| !l.is_empty()) {
        let mut key = String::new();
        for part in line.split_ascii_whitespace() {
            if part.ends_with(':') {
                key = part.trim_end_matches(':').to_string();
            } else {
                output
                    .entry(key.clone())
                    .and_modify(|v| v.push(part.to_string()))
                    .or_insert(vec![part.to_string()]);
            }
        }
    }

    output
}

fn part1(connections: &HashMap<String, Vec<String>>) -> u64 {
    let mut nof_paths_to_out = 0;

    let mut open_nodes = vec!["you".to_string()];

    while let Some(node) = open_nodes.pop() {
        for connected_node in &connections[&node] {
            if connected_node == "out" {
                nof_paths_to_out += 1;
            } else {
                open_nodes.push(connected_node.to_string());
            }
        }
    }

    nof_paths_to_out
}

fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("reading input must work");
    let connections = parse_input(&input);

    println!("Part 1: {}", part1(&connections));
}

#[test]
fn test_part1() {
    let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

    let connections = parse_input(input);

    assert_eq!(part1(&connections), 5);
}