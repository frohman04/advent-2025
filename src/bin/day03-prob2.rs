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

fn find_max_joltage_all(banks: Vec<Vec<u8>>) -> u64 {
    banks.into_iter().map(find_max_joltage_bank).sum()
}

fn find_max_joltage_bank(bank: Vec<u8>) -> u64 {
    let digits = 12;
    let mut max_val = 0u64;
    let mut start_i = 0usize;

    for digit in (0..digits).rev() {
        let mut max_digit: u8 = 0;
        let mut max_digit_i: usize = 0;

        for (i, item) in bank
            .iter()
            .enumerate()
            .take(bank.len() - digit)
            .skip(start_i)
        {
            if item > &max_digit {
                max_digit = *item;
                max_digit_i = i;
            }
        }

        max_val += max_digit as u64 * 10u64.pow(digit as u32);
        start_i = max_digit_i + 1;
    }

    max_val
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
            987654321111
        )
    }

    #[test]
    fn test_find_max_joltage_bank_2() {
        assert_eq!(
            find_max_joltage_bank(vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]),
            811111111119
        )
    }

    #[test]
    fn test_find_max_joltage_bank_3() {
        assert_eq!(
            find_max_joltage_bank(vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]),
            434234234278
        )
    }

    #[test]
    fn test_find_max_joltage_bank_4() {
        assert_eq!(
            find_max_joltage_bank(vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1]),
            888911112111
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
            3121910778619
        )
    }
}
