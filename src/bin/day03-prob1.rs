fn main() {
    let banks = std::fs::read_to_string("src/bin/day03.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(parse_line)
                .collect()
        })
        .expect("Unable to open file");
    println!("{}", find_max_joltage_all(banks))
}

fn parse_line(line: &str) -> Vec<u8> {
    line.chars()
        .map(|char| {
            char.to_string()
                .parse::<u8>()
                .expect("unable to parse number")
        })
        .collect()
}

fn find_max_joltage_all(banks: Vec<Vec<u8>>) -> u16 {
    banks
        .into_iter()
        .map(|bank| find_max_joltage_bank(bank) as u16)
        .sum()
}

fn find_max_joltage_bank(bank: Vec<u8>) -> u8 {
    let mut max = 0u8;
    for i in 0..bank.len() - 1 {
        for j in i + 1..bank.len() {
            let val = bank[i] * 10 + bank[j];
            if val > max {
                max = val;
            }
        }
    }
    max
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("987654321111111"),
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]
        )
    }

    #[test]
    fn test_find_max_joltage_bank_1() {
        assert_eq!(
            find_max_joltage_bank(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]),
            98
        )
    }

    #[test]
    fn test_find_max_joltage_bank_2() {
        assert_eq!(
            find_max_joltage_bank(vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]),
            89
        )
    }

    #[test]
    fn test_find_max_joltage_bank_3() {
        assert_eq!(
            find_max_joltage_bank(vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]),
            78
        )
    }

    #[test]
    fn test_find_max_joltage_bank_4() {
        assert_eq!(
            find_max_joltage_bank(vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]),
            92
        )
    }

    #[test]
    fn test_find_max_joltage_all() {
        assert_eq!(
            find_max_joltage_all(vec![
                vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
                vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
                vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
                vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]
            ]),
            357
        )
    }
}
