use std::collections::VecDeque;

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
    pub indicators: u16,
    pub schematics: Vec<u16>,
    pub requirements: Vec<u32>,
}

#[inline]
fn get_bitmask_bool(mask: &[bool]) -> u16 {
    let mut mask_val: u16 = 0;
    for (i, val) in mask.iter().enumerate() {
        if *val {
            mask_val |= 2u16.pow(i as u32);
        }
    }
    mask_val
}

#[inline]
fn get_bitmask_indexes(mask: &[usize]) -> u16 {
    let mut mask_val: u16 = 0;
    for i in mask.iter() {
        mask_val |= 2u16.pow(*i as u32)
    }
    mask_val
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
        indicators: get_bitmask_bool(&indicators),
        schematics: schematics
            .into_iter()
            .map(|val| get_bitmask_indexes(&val))
            .collect(),
        requirements,
    }
}

#[derive(Debug)]
struct SolState {
    cost: usize,
    indicators: u16,
}

fn turn_on_machine(machine: Machine) -> usize {
    let mut queue: VecDeque<SolState> = VecDeque::new();
    queue.push_back(SolState {
        cost: 0,
        indicators: 0,
    });

    while let Some(SolState { cost, indicators }) = queue.pop_front() {
        if indicators == machine.indicators {
            return cost;
        }
        for schematic in machine.schematics.iter() {
            let new_indicators = indicators ^ schematic;
            queue.push_back(SolState {
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
    fn test_get_bitmask_bool() {
        assert_eq!(get_bitmask_bool(&vec![true, true, false, false]), 0b0011)
    }

    #[test]
    fn test_get_bitmask_indexes() {
        assert_eq!(get_bitmask_indexes(&vec![2, 3]), 0b1100)
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"),
            Machine {
                indicators: 0b0110,
                schematics: vec![0b1000, 0b1010, 0b0100, 0b1100, 0b0101, 0b0011],
                requirements: vec![3, 5, 4, 7]
            }
        )
    }

    #[test]
    fn test_turn_on_machine_1() {
        assert_eq!(
            turn_on_machine(Machine {
                indicators: 0b0110,
                schematics: vec![0b1000, 0b1010, 0b0100, 0b1100, 0b0101, 0b0011],
                requirements: vec![3, 5, 4, 7]
            }),
            2
        )
    }

    #[test]
    fn test_turn_on_machine_2() {
        assert_eq!(
            turn_on_machine(Machine {
                indicators: 0b01000,
                schematics: vec![0b11101, 0b01100, 0b10001, 0b00111, 0b11110],
                requirements: vec![7, 5, 12, 7, 2]
            }),
            3
        )
    }

    #[test]
    fn test_turn_on_machine_3() {
        assert_eq!(
            turn_on_machine(Machine {
                indicators: 0b101110,
                schematics: vec![0b011111, 0b011001, 0b110111, 0b000110],
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
                    indicators: 0b0110,
                    schematics: vec![0b1000, 0b1010, 0b0100, 0b1100, 0b0101, 0b0011],
                    requirements: vec![3, 5, 4, 7]
                },
                Machine {
                    indicators: 0b01000,
                    schematics: vec![0b11101, 0b01100, 0b10001, 0b00111, 0b11110,],
                    requirements: vec![7, 5, 12, 7, 2]
                },
                Machine {
                    indicators: 0b101110,
                    schematics: vec![0b011111, 0b011001, 0b110111, 0b000110,],
                    requirements: vec![10, 11, 11, 5, 10, 5]
                }
            ]),
            7
        )
    }
}
