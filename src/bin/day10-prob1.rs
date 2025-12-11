use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn main() {
    let machines = std::fs::read_to_string("src/bin/day10.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(parse_line)
                .collect()
        })
        .expect("Unable to open file");
    println!("{}", turn_on_all(machines))
}

#[derive(PartialEq, Debug)]
struct Machine {
    pub indicators: Vec<bool>,
    pub schematics: Vec<Vec<usize>>,
    pub requirements: Vec<u32>,
}

fn parse_line(line: &str) -> Machine {
    let mut indicators: Vec<bool> = Vec::new();
    let mut schematics: Vec<Vec<usize>> = Vec::new();
    let mut requirements: Vec<u32> = Vec::new();

    let mut in_indicators = false;
    let mut in_schematic = false;
    let mut in_requirements = false;
    let mut buffer: Vec<char> = Vec::new();
    for char in line.chars() {
        if char == ' ' {
            continue;
        }

        if (if in_indicators { 1 } else { 0 })
            + (if in_schematic { 1 } else { 0 })
            + (if in_requirements { 1 } else { 0 })
            > 1
        {
            panic!(
                "Invalid state:\n\tin_indicators = {}\n\tin_schematic = {}\n\tin_requirements = {}",
                in_indicators, in_schematic, in_requirements
            )
        }

        if !in_indicators && !in_schematic && !in_requirements {
            match char {
                '[' => in_indicators = true,
                '(' => in_schematic = true,
                '{' => in_requirements = true,
                x => panic!(
                    "Found unexpected character outside of capture context: {}",
                    x
                ),
            }
        } else if in_indicators {
            if char == ']' {
                for item in buffer.iter() {
                    indicators.push(*item == '#');
                }

                buffer.clear();
                in_indicators = false;
            } else {
                buffer.push(char);
            }
        } else if in_schematic {
            if char == ')' {
                let val: String = buffer.iter().collect();
                let nums: Vec<usize> = val
                    .split(",")
                    .map(|v| v.parse().expect("Unable to parse number"))
                    .collect();
                schematics.push(nums);

                buffer.clear();
                in_schematic = false;
            } else {
                buffer.push(char);
            }
        } else if in_requirements {
            if char == '}' {
                let val: String = buffer.iter().collect();
                val.split(",")
                    .map(|v| v.parse().expect("Unable to parse number"))
                    .for_each(|v| requirements.push(v));

                buffer.clear();
                in_requirements = false;
            } else {
                buffer.push(char);
            }
        }
    }

    Machine {
        indicators,
        schematics,
        requirements,
    }
}

#[derive(Eq, PartialEq, Debug)]
struct SolState {
    cost: usize,
    indicators: Vec<bool>,
}

impl Ord for SolState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.indicators.cmp(&other.indicators))
    }
}

impl PartialOrd for SolState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn turn_on_machine(machine: Machine) -> usize {
    let mut queue: BinaryHeap<SolState> = BinaryHeap::new();
    queue.push(SolState {
        cost: 0,
        indicators: vec![false; machine.indicators.len()],
    });

    while let Some(SolState { cost, indicators }) = queue.pop() {
        if indicators == machine.indicators {
            return cost;
        }
        for schematic in machine.schematics.iter() {
            let mut new_indicators = indicators.clone();
            for light_i in schematic {
                new_indicators[*light_i] = !new_indicators[*light_i];
            }
            queue.push(SolState {
                cost: cost + 1,
                indicators: new_indicators,
            });
        }
    }

    panic!("Unable to find solution!")
}

fn turn_on_all(machines: Vec<Machine>) -> usize {
    machines
        .into_iter()
        .enumerate()
        .map(|(i, machine)| {
            let count = turn_on_machine(machine);
            println!("Solved machine {}, result {}", i, count);
            count
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"),
            Machine {
                indicators: vec![false, true, true, false],
                schematics: vec![
                    vec![3],
                    vec![1, 3],
                    vec![2],
                    vec![2, 3],
                    vec![0, 2],
                    vec![0, 1],
                ],
                requirements: vec![3, 5, 4, 7]
            }
        )
    }

    #[test]
    fn test_turn_on_machine_1() {
        assert_eq!(
            turn_on_machine(Machine {
                indicators: vec![false, true, true, false],
                schematics: vec![
                    vec![3],
                    vec![1, 3],
                    vec![2],
                    vec![2, 3],
                    vec![0, 2],
                    vec![0, 1],
                ],
                requirements: vec![3, 5, 4, 7]
            }),
            2
        )
    }

    #[test]
    fn test_turn_on_machine_2() {
        assert_eq!(
            turn_on_machine(Machine {
                indicators: vec![false, false, false, true, false],
                schematics: vec![
                    vec![0, 2, 3, 4],
                    vec![2, 3],
                    vec![0, 4],
                    vec![0, 1, 2],
                    vec![1, 2, 3, 4],
                ],
                requirements: vec![7, 5, 12, 7, 2]
            }),
            3
        )
    }

    #[test]
    fn test_turn_on_machine_3() {
        assert_eq!(
            turn_on_machine(Machine {
                indicators: vec![false, true, true, true, false, true],
                schematics: vec![
                    vec![0, 1, 2, 3, 4],
                    vec![0, 3, 4],
                    vec![0, 1, 2, 4, 5],
                    vec![1, 2],
                ],
                requirements: vec![10, 11, 11, 5, 10, 5]
            }),
            2
        )
    }

    #[test]
    fn test_turn_on_all() {
        assert_eq!(
            turn_on_all(vec![
                Machine {
                    indicators: vec![false, true, true, false],
                    schematics: vec![
                        vec![3],
                        vec![1, 3],
                        vec![2],
                        vec![2, 3],
                        vec![0, 2],
                        vec![0, 1],
                    ],
                    requirements: vec![3, 5, 4, 7]
                },
                Machine {
                    indicators: vec![false, false, false, true, false],
                    schematics: vec![
                        vec![0, 2, 3, 4],
                        vec![2, 3],
                        vec![0, 4],
                        vec![0, 1, 2],
                        vec![1, 2, 3, 4],
                    ],
                    requirements: vec![7, 5, 12, 7, 2]
                },
                Machine {
                    indicators: vec![false, true, true, true, false, true],
                    schematics: vec![
                        vec![0, 1, 2, 3, 4],
                        vec![0, 3, 4],
                        vec![0, 1, 2, 4, 5],
                        vec![1, 2],
                    ],
                    requirements: vec![10, 11, 11, 5, 10, 5]
                }
            ]),
            7
        )
    }
}
