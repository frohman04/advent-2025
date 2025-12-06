use regex::Regex;

fn main() {
    let (num_lines, op_line) = std::fs::read_to_string("src/bin/day06.txt")
        .map(|file| parse_lines(file.lines().collect()))
        .expect("Unable to open file");
    println!("{}", do_math(num_lines, op_line))
}

#[derive(PartialEq, Debug)]
enum Op {
    Add,
    Mul,
}

fn parse_lines(lines: Vec<&str>) -> (Vec<Vec<u64>>, Vec<Op>) {
    let separator = Regex::new(r"\s+").expect("Invalid regex");
    let parsed_lines: Vec<Vec<&str>> = lines
        .into_iter()
        .map(|line| separator.split(line.trim()).collect())
        .collect();

    let op_line: Vec<Op> = parsed_lines[parsed_lines.len() - 1]
        .iter()
        .map(|raw_op| match *raw_op {
            "*" => Op::Mul,
            "+" => Op::Add,
            x => panic!("Unknown operation: {}", x),
        })
        .collect();
    let num_lines: Vec<Vec<u64>> = parsed_lines[..parsed_lines.len() - 1]
        .iter()
        .map(|line| {
            line.iter()
                .map(|raw_num| raw_num.parse().expect("Unparsable number"))
                .collect()
        })
        .collect();

    for num_line in num_lines.iter() {
        if op_line.len() != num_line.len() {
            panic!("Lines must be same length")
        }
    }

    (num_lines, op_line)
}

fn do_math(num_lines: Vec<Vec<u64>>, op_line: Vec<Op>) -> u64 {
    op_line
        .into_iter()
        .enumerate()
        .map(|(i, op)| {
            let input = num_lines.iter().map(|num_line| num_line[i]);
            match op {
                Op::Add => input.sum::<u64>(),
                Op::Mul => input.product(),
            }
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_lines() {
        assert_eq!(
            parse_lines(vec![
                "123 328  51 64 ",
                " 45 64  387 23 ",
                "  6 98  215 314",
                "*   +   *   +  ",
            ]),
            (
                vec![
                    vec![123, 328, 51, 64],
                    vec![45, 64, 387, 23],
                    vec![6, 98, 215, 314]
                ],
                vec![Op::Mul, Op::Add, Op::Mul, Op::Add]
            )
        )
    }

    #[test]
    fn test_do_math() {
        assert_eq!(
            do_math(
                vec![
                    vec![123, 328, 51, 64],
                    vec![45, 64, 387, 23],
                    vec![6, 98, 215, 314]
                ],
                vec![Op::Mul, Op::Add, Op::Mul, Op::Add]
            ),
            4277556
        )
    }
}
