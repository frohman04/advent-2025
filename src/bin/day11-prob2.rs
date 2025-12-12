use std::collections::HashMap;

fn main() {
    let network = std::fs::read_to_string("src/bin/day11.txt")
        .map(|file| parse_lines(file.lines().collect()))
        .expect("Unable to open file");
    println!("{}", find_paths(network))
}

#[derive(PartialEq, Eq, Debug)]
struct Network {
    node_ids: HashMap<String, usize>,
    connections: HashMap<usize, Vec<usize>>,
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
    let mut node_ids: HashMap<String, usize> = HashMap::new();
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

    let connections: HashMap<usize, Vec<usize>> = raw_map
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

fn _find_paths(
    network: &Network,
    node_id: usize,
    target_node_id: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    let key = (node_id, target_node_id);
    if cache.contains_key(&key) {
        *cache.get(&key).unwrap()
    } else {
        let new_val = if node_id == target_node_id {
            1
        } else {
            network
                .connections
                .get(&node_id)
                .unwrap_or(&vec![])
                .iter()
                .map(|next_node_id| _find_paths(network, *next_node_id, target_node_id, cache))
                .sum()
        };

        cache.insert(key, new_val);
        new_val
    }
}

#[allow(clippy::unnecessary_to_owned)]
fn find_paths(network: Network) -> usize {
    let svr_i = *network.node_ids.get(&"svr".to_string()).unwrap();
    let dac_i = *network.node_ids.get(&"dac".to_string()).unwrap();
    let fft_i = *network.node_ids.get(&"fft".to_string()).unwrap();
    let out_i = *network.node_ids.get(&"out".to_string()).unwrap();

    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
    let svr_to_dac = _find_paths(&network, svr_i, dac_i, &mut cache);
    dbg!(svr_to_dac);
    let dac_to_fft = _find_paths(&network, dac_i, fft_i, &mut cache);
    dbg!(dac_to_fft);
    let fft_to_out = _find_paths(&network, fft_i, out_i, &mut cache);
    dbg!(fft_to_out);
    let svr_to_fft = _find_paths(&network, svr_i, fft_i, &mut cache);
    dbg!(svr_to_fft);
    let fft_to_dac = _find_paths(&network, fft_i, dac_i, &mut cache);
    dbg!(fft_to_dac);
    let dac_to_out = _find_paths(&network, dac_i, out_i, &mut cache);
    dbg!(dac_to_out);
    (svr_to_dac * dac_to_fft * fft_to_out) + (svr_to_fft * fft_to_dac * dac_to_out)
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
                    "svr".to_string() => 0,
                    "aaa".to_string() => 1,
                    "bbb".to_string() => 2,
                    "fft".to_string() => 3,
                    "ccc".to_string() => 4,
                    "tty".to_string() => 5,
                    "ddd".to_string() => 6,
                    "eee".to_string() => 7,
                    "hub".to_string() => 8,
                    "fff".to_string() => 9,
                    "dac".to_string() => 10,
                    "ggg".to_string() => 11,
                    "hhh".to_string() => 12,
                    "out".to_string() => 13,
                },
                connections: hashmap! {
                    0 => vec![1, 2],
                    1 => vec![3],
                    3 => vec![4],
                    2 => vec![5],
                    5 => vec![4],
                    4 => vec![6, 7],
                    6 => vec![8],
                    8 => vec![9],
                    7 => vec![10],
                    10 => vec![9],
                    9 => vec![11, 12],
                    11 => vec![13],
                    12 => vec![13],
                }
            }),
            2
        )
    }
}
