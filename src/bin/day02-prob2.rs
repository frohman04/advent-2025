fn main() {
    let ranges = std::fs::read_to_string("src/bin/day02.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(parse_line)
                .next()
                .expect("Should be a line in the input")
        })
        .expect("Unable to open file");
    println!("{}", find_invalid_ids_all(ranges))
}

fn parse_line(line: &str) -> Vec<(u64, u64)> {
    line.split(",")
        .map(|chunk| {
            let nums: Vec<u64> = chunk
                .split("-")
                .map(|raw_num| raw_num.parse::<u64>().expect("Unable to parse number"))
                .collect();
            (nums[0], nums[1])
        })
        .collect()
}

fn find_invalid_ids_all(ranges: Vec<(u64, u64)>) -> u64 {
    ranges.into_iter().flat_map(find_invalid_ids).sum()
}

fn find_invalid_ids(range: (u64, u64)) -> Vec<u64> {
    (range.0..=range.1)
        .filter(|num| {
            let usable = num.to_string();
            let chars = usable.as_bytes();
            (2..=usable.len())
                .filter(|num_pieces| usable.len() % num_pieces == 0)
                .any(|num_pieces| {
                    let piece_size = usable.len() / num_pieces;
                    (0..piece_size).all(|offset| {
                        let first = chars[offset];
                        (0..num_pieces).all(|piece_i| first == chars[piece_size * piece_i + offset])
                    })
                })
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("11-22,95-115,2121212118-2121212124"),
            vec![(11, 22), (95, 115), (2121212118, 2121212124)]
        )
    }

    #[test]
    fn test_find_invalid_ids() {
        assert_eq!(find_invalid_ids((11, 22)), vec![11, 22])
    }

    #[test]
    fn test_find_invalid_ids2() {
        assert_eq!(find_invalid_ids((95, 115)), vec![99, 111])
    }

    #[test]
    fn test_find_invalid_ids3() {
        assert_eq!(find_invalid_ids((565653, 565659)), vec![565656])
    }

    #[test]
    fn test_find_invalid_ids_all() {
        assert_eq!(
            find_invalid_ids_all(vec![
                (11, 22),
                (95, 115),
                (998, 1012),
                (1188511880, 1188511890),
                (222220, 222224),
                (1698522, 1698528),
                (446443, 446449),
                (38593856, 38593862),
                (565653, 565659),
                (824824821, 824824827),
                (2121212118, 2121212124)
            ]),
            4174379265
        )
    }
}
