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
    let op_line: Vec<(usize, Op)> = lines[lines.len() - 1]
        .chars()
        .enumerate()
        .filter(|(_, char)| *char != ' ')
        .map(|(i, char)| {
            (
                i,
                match char {
                    '*' => Op::Mul,
                    '+' => Op::Add,
                    x => panic!("Unknown operation: {}", x),
                },
            )
        })
        .collect();

    let char_lines: Vec<Vec<char>> = lines[..lines.len() - 1]
        .iter()
        .map(|line| line.chars().collect())
        .collect();
    let mut right_limit = char_lines[0].len();
    let mut nums_for_op: Vec<Vec<u64>> = vec![vec![]; op_line.len()];
    for (op_i, (left_limit, _)) in op_line.iter().enumerate().rev() {
        let mut nums: Vec<u64> = Vec::new();
        for col_i in (*left_limit..right_limit).rev() {
            let raw_num: String = char_lines.iter().map(|row| row[col_i]).collect();
            let num: u64 = raw_num.trim().parse().expect("Unable to parse number");
            nums.push(num);
        }
        nums_for_op[op_i] = nums;

        if *left_limit > 0 {
            right_limit = *left_limit - 1;
        }
    }

    (nums_for_op, op_line.into_iter().map(|(_, op)| op).collect())
}

fn do_math(num_lines: Vec<Vec<u64>>, op_line: Vec<Op>) -> u64 {
    op_line
        .into_iter()
        .enumerate()
        .map(|(i, op)| match op {
            Op::Add => num_lines[i].iter().sum::<u64>(),
            Op::Mul => num_lines[i].iter().product(),
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
                    vec![356, 24, 1],
                    vec![8, 248, 369],
                    vec![175, 581, 32],
                    vec![4, 431, 623],
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
                    vec![356, 24, 1],
                    vec![8, 248, 369],
                    vec![175, 581, 32],
                    vec![4, 431, 623],
                ],
                vec![Op::Mul, Op::Add, Op::Mul, Op::Add]
            ),
            3263827
        )
    }
}
