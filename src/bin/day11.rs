use std::{
    collections::HashMap,
    io::{Read, stdin},
};

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

fn cached_dfs(
    node: (String, bool, bool),
    cache: &mut HashMap<(String, bool, bool), u64>,
    connections: &HashMap<String, Vec<String>>,
) -> u64 {
    if let Some(result) = cache.get(&node) {
        return *result;
    }

    let mut result = 0;

    let (node, has_seen_fft, has_seen_dac) = node;

    for connected_node in &connections[&node] {
        if connected_node == "out" {
            if has_seen_fft && has_seen_dac {
                result += 1;
            }
        } else {
            let has_seen_fft = has_seen_fft || (connected_node == "fft");
            let has_seen_dac = has_seen_dac || (connected_node == "dac");
            result += cached_dfs(
                (connected_node.to_string(), has_seen_fft, has_seen_dac),
                cache,
                connections,
            );
        }
    }

    cache.insert((node, has_seen_fft, has_seen_dac), result);
    result
}

fn part2(connections: &HashMap<String, Vec<String>>) -> u64 {
    let mut cache = HashMap::new();

    cached_dfs(("svr".to_string(), false, false), &mut cache, connections)
}

fn main() {
    let mut input = String::new();
    stdin()
        .read_to_string(&mut input)
        .expect("reading input must work");
    let connections = parse_input(&input);

    println!("Part 1: {}", part1(&connections));
    println!("Part 2: {}", part2(&connections));
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

#[test]
fn test_part2() {
    let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

    let connections = parse_input(input);

    assert_eq!(part2(&connections), 2);
}
