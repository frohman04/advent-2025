use std::cmp::PartialOrd;
use std::collections::HashMap;
use std::rc::Rc;

fn main() {
    let boxes: Vec<Pos> = std::fs::read_to_string("src/bin/day08.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(parse_line)
                .collect()
        })
        .expect("Unable to open file");
    println!("{}", build_circuits(boxes))
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Debug)]
struct Pos {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

fn parse_line(line: &str) -> Pos {
    let parts: Vec<u32> = line
        .split(",")
        .map(|num| num.parse().expect("unable to parse number"))
        .collect();
    Pos {
        x: parts[0],
        y: parts[1],
        z: parts[2],
    }
}

fn distance(pos1: &Pos, pos2: &Pos) -> f32 {
    (((pos1.x as i64 - pos2.x as i64).pow(2)
        + (pos1.y as i64 - pos2.y as i64).pow(2)
        + (pos1.z as i64 - pos2.z as i64).pow(2)) as f32)
        .sqrt()
}

fn calc_distances(boxes: Vec<Pos>) -> Vec<(f32, Rc<Pos>, Rc<Pos>)> {
    let rc_boxes: Vec<Rc<Pos>> = boxes.into_iter().map(Rc::new).collect();
    let mut out_boxes: Vec<(f32, Rc<Pos>, Rc<Pos>)> = rc_boxes
        .iter()
        .enumerate()
        .flat_map(|(i, box1)| {
            rc_boxes
                .iter()
                .skip(i + 1)
                .map(|box2| {
                    if box1 < box2 {
                        (distance(box1, box2), box1.clone(), box2.clone())
                    } else {
                        (distance(box2, box1), box2.clone(), box1.clone())
                    }
                })
                .collect::<Vec<(f32, Rc<Pos>, Rc<Pos>)>>()
        })
        .collect();
    out_boxes.sort_by(|a, b| b.partial_cmp(a).unwrap());
    out_boxes
}

fn build_circuits(boxes: Vec<Pos>) -> u64 {
    let mut box_distances = calc_distances(boxes);
    let mut circuits: HashMap<Rc<Pos>, u32> = HashMap::with_capacity(box_distances.len());
    let mut circuit_counter: u32 = 0;

    let mut most_recent_connected: Option<(Rc<Pos>, Rc<Pos>)> = None;
    while let Some((_, box1, box2)) = box_distances.pop() {
        let to_update: Vec<(Rc<Pos>, u32)> = match (circuits.get(&box1), circuits.get(&box2)) {
            (Some(b1), Some(b2)) if b1 != b2 => {
                most_recent_connected = Some((box1.clone(), box2.clone()));
                circuits
                    .iter()
                    .filter(|(_, value)| *value == b2)
                    .map(|(key, _)| (key.clone(), *b1))
                    .collect()
            }
            (Some(b1), None) => {
                most_recent_connected = Some((box1.clone(), box2.clone()));
                vec![(box2, *b1)]
            }
            (None, Some(b2)) => {
                most_recent_connected = Some((box1.clone(), box2.clone()));
                vec![(box1, *b2)]
            }
            (None, None) => {
                most_recent_connected = Some((box1.clone(), box2.clone()));
                let id = circuit_counter;
                circuit_counter += 1;
                vec![(box1.clone(), id), (box2.clone(), id)]
            }
            _ => vec![],
        };
        to_update.into_iter().for_each(|(key, value)| {
            circuits.insert(key, value);
        });
    }
    {
        println!("{{");
        let mut sorted_circuits: Vec<(&Rc<Pos>, &u32)> = circuits.iter().collect();
        sorted_circuits
            .sort_by(|(pos1, i1), (pos2, i2)| (i1, pos1).partial_cmp(&(i2, pos2)).unwrap());
        for (pos, i) in sorted_circuits {
            println!("\t{}: {:?}", i, pos);
        }
        println!("}}");
    }

    if let Some((b1, b2)) = most_recent_connected {
        b1.x as u64 * b2.x as u64
    } else {
        panic!("Never connected any boxes!")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("162,817,812"),
            Pos {
                x: 162,
                y: 817,
                z: 812
            }
        )
    }

    #[test]
    fn test_calc_distances() {
        assert_eq!(
            calc_distances(vec![
                Pos {
                    x: 162,
                    y: 817,
                    z: 812
                },
                Pos {
                    x: 425,
                    y: 690,
                    z: 689
                },
                Pos {
                    x: 431,
                    y: 825,
                    z: 988
                },
                Pos {
                    x: 906,
                    y: 360,
                    z: 560
                },
                Pos {
                    x: 805,
                    y: 96,
                    z: 715
                },
            ])
            .into_iter()
            .rev()
            .take(4)
            .rev()
            .map(|(_, box1, box2)| (box1, box2))
            .collect::<Vec<(Rc<Pos>, Rc<Pos>)>>(),
            vec![
                (
                    Rc::new(Pos {
                        x: 425,
                        y: 690,
                        z: 689
                    }),
                    Rc::new(Pos {
                        x: 431,
                        y: 825,
                        z: 988
                    })
                ),
                (
                    Rc::new(Pos {
                        x: 805,
                        y: 96,
                        z: 715
                    }),
                    Rc::new(Pos {
                        x: 906,
                        y: 360,
                        z: 560
                    })
                ),
                (
                    Rc::new(Pos {
                        x: 162,
                        y: 817,
                        z: 812
                    }),
                    Rc::new(Pos {
                        x: 431,
                        y: 825,
                        z: 988
                    })
                ),
                (
                    Rc::new(Pos {
                        x: 162,
                        y: 817,
                        z: 812
                    }),
                    Rc::new(Pos {
                        x: 425,
                        y: 690,
                        z: 689
                    })
                )
            ]
        )
    }

    #[test]
    fn test_build_circuits() {
        assert_eq!(
            build_circuits(vec![
                Pos {
                    x: 162,
                    y: 817,
                    z: 812,
                },
                Pos {
                    x: 57,
                    y: 618,
                    z: 57,
                },
                Pos {
                    x: 906,
                    y: 360,
                    z: 560,
                },
                Pos {
                    x: 592,
                    y: 479,
                    z: 940,
                },
                Pos {
                    x: 352,
                    y: 342,
                    z: 300,
                },
                Pos {
                    x: 466,
                    y: 668,
                    z: 158,
                },
                Pos {
                    x: 542,
                    y: 29,
                    z: 236,
                },
                Pos {
                    x: 431,
                    y: 825,
                    z: 988,
                },
                Pos {
                    x: 739,
                    y: 650,
                    z: 466,
                },
                Pos {
                    x: 52,
                    y: 470,
                    z: 668,
                },
                Pos {
                    x: 216,
                    y: 146,
                    z: 977,
                },
                Pos {
                    x: 819,
                    y: 987,
                    z: 18,
                },
                Pos {
                    x: 117,
                    y: 168,
                    z: 530,
                },
                Pos {
                    x: 805,
                    y: 96,
                    z: 715,
                },
                Pos {
                    x: 346,
                    y: 949,
                    z: 466,
                },
                Pos {
                    x: 970,
                    y: 615,
                    z: 88,
                },
                Pos {
                    x: 941,
                    y: 993,
                    z: 340,
                },
                Pos {
                    x: 862,
                    y: 61,
                    z: 35,
                },
                Pos {
                    x: 984,
                    y: 92,
                    z: 344,
                },
                Pos {
                    x: 425,
                    y: 690,
                    z: 689,
                },
            ]),
            25272
        )
    }
}
