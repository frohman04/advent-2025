use std::collections::{HashMap, VecDeque};

fn main() {
    let network = std::fs::read_to_string("src/bin/day11.txt")
        .map(|file| parse_lines(file.lines().collect()))
        .expect("Unable to open file");
    println!("{}", find_paths(network))
}

#[derive(PartialEq, Eq, Debug)]
struct Network {
    node_ids: HashMap<String, u16>,
    connections: HashMap<u16, Vec<u16>>,
}

#[allow(clippy::unnecessary_to_owned)]
fn parse_lines(lines: Vec<&str>) -> Network {
    let raw_map: Vec<(&str, Vec<&str>)> = lines
        .into_iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(":").collect();
            (parts[0], parts[1].trim().split(" ").collect())
        })
        .collect();

    let mut curr_id = 0;
    let mut node_ids: HashMap<String, u16> = HashMap::new();
    for (source_node, sink_nodes) in raw_map.iter() {
        if let std::collections::hash_map::Entry::Vacant(e) =
            node_ids.entry(source_node.to_string())
        {
            e.insert(curr_id);
            curr_id += 1;
        }

        for sink_node in sink_nodes {
            if let std::collections::hash_map::Entry::Vacant(e) =
                node_ids.entry(sink_node.to_string())
            {
                e.insert(curr_id);
                curr_id += 1;
            }
        }
    }

    let connections: HashMap<u16, Vec<u16>> = raw_map
        .iter()
        .map(|(source_node, sink_nodes)| {
            (
                *node_ids
                    .get(&source_node.to_string())
                    .expect("Unable to find value"),
                sink_nodes
                    .iter()
                    .map(|sink_node| {
                        *node_ids
                            .get(&sink_node.to_string())
                            .expect("Unable to find value")
                    })
                    .collect(),
            )
        })
        .collect();

    Network {
        node_ids,
        connections,
    }
}

#[allow(clippy::unnecessary_to_owned)]
fn find_paths(network: Network) -> usize {
    let mut queue: VecDeque<u16> = VecDeque::new();
    queue.push_back(*network.node_ids.get(&"you".to_string()).unwrap());

    let target_node_id = *network.node_ids.get(&"out".to_string()).unwrap();
    let mut path_count: usize = 0;

    while let Some(node_id) = queue.pop_front() {
        if node_id == target_node_id {
            path_count += 1;
        } else {
            for next_node_id in network.connections.get(&node_id).unwrap() {
                queue.push_back(*next_node_id);
            }
        }
    }

    path_count
}

#[cfg(test)]
mod test {
    use super::*;
    use maplit::hashmap;

    #[test]
    fn test_parse_lines() {
        assert_eq!(
            parse_lines(vec![
                "aaa: you hhh",
                "you: bbb ccc",
                "bbb: ddd eee",
                "ccc: ddd eee fff",
                "ddd: ggg",
                "eee: out",
                "fff: out",
                "ggg: out",
                "hhh: ccc fff iii",
                "iii: out",
            ]),
            Network {
                node_ids: hashmap! {
                    "aaa".to_string() => 0,
                    "you".to_string() => 1,
                    "hhh".to_string() => 2,
                    "bbb".to_string() => 3,
                    "ccc".to_string() => 4,
                    "ddd".to_string() => 5,
                    "eee".to_string() => 6,
                    "fff".to_string() => 7,
                    "ggg".to_string() => 8,
                    "out".to_string() => 9,
                    "iii".to_string() => 10,
                },
                connections: hashmap! {
                    0 => vec![1, 2],
                    1 => vec![3, 4],
                    3 => vec![5, 6],
                    4 => vec![5, 6, 7],
                    5 => vec![8],
                    6 => vec![9],
                    7 => vec![9],
                    8 => vec![9],
                    2 => vec![4, 7, 10],
                    10 => vec![9],
                }
            }
        )
    }

    #[test]
    fn test_calc_sizes() {
        assert_eq!(
            find_paths(Network {
                node_ids: hashmap! {
                    "aaa".to_string() => 0,
                    "you".to_string() => 1,
                    "hhh".to_string() => 2,
                    "bbb".to_string() => 3,
                    "ccc".to_string() => 4,
                    "ddd".to_string() => 5,
                    "eee".to_string() => 6,
                    "fff".to_string() => 7,
                    "ggg".to_string() => 8,
                    "out".to_string() => 9,
                    "iii".to_string() => 10,
                },
                connections: hashmap! {
                    0 => vec![1, 2],
                    1 => vec![3, 4],
                    3 => vec![5, 6],
                    4 => vec![5, 6, 7],
                    5 => vec![8],
                    6 => vec![9],
                    7 => vec![9],
                    8 => vec![9],
                    2 => vec![4, 7, 10],
                    10 => vec![9],
                }
            }),
            5
        )
    }
}
