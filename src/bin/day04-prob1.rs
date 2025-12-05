fn main() {
    let banks = std::fs::read_to_string("src/bin/day04.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(parse_line)
                .collect()
        })
        .expect("Unable to open file");
    println!("{}", count_accessible_rolls(banks))
}

fn parse_line(line: &str) -> Vec<bool> {
    line.chars().map(|char| char == '@').collect()
}

fn count_accessible_rolls(map: Vec<Vec<bool>>) -> usize {
    let mut neighbor_count = vec![vec![0u8; map[0].len()]; map.len()];
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] {
                if i > 0 && j > 0 {
                    neighbor_count[i - 1][j - 1] += 1;
                }
                if i > 0 {
                    neighbor_count[i - 1][j] += 1;
                }
                if j > 0 {
                    neighbor_count[i][j - 1] += 1;
                }
                if j + 1 < map[i].len() {
                    neighbor_count[i][j + 1] += 1;
                }
                if i + 1 < map.len() {
                    neighbor_count[i + 1][j] += 1;
                }
                if i + 1 < map.len() && j + 1 < map[i].len() {
                    neighbor_count[i + 1][j + 1] += 1;
                }
                if i + 1 < map.len() && j > 0 {
                    neighbor_count[i + 1][j - 1] += 1;
                }
                if i > 0 && j + 1 < map[i].len() {
                    neighbor_count[i - 1][j + 1] += 1;
                }
            }
        }
    }

    let mut count: usize = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] && neighbor_count[i][j] < 4 {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("..@@.@@@@."),
            vec![
                false, false, true, true, false, true, true, true, true, false
            ]
        )
    }

    #[test]
    fn test_count_accessible_rolls() {
        let f = false;
        let t = true;
        assert_eq!(
            count_accessible_rolls(vec![
                vec![f, f, t, t, f, t, t, t, t, f],
                vec![t, t, t, f, t, f, t, f, t, t],
                vec![t, t, t, t, t, f, t, f, t, t],
                vec![t, f, t, t, t, t, f, f, t, f],
                vec![t, t, f, t, t, t, t, f, t, t],
                vec![f, t, t, t, t, t, t, t, f, t],
                vec![f, t, f, t, f, t, f, t, t, t],
                vec![t, f, t, t, t, f, t, t, t, t],
                vec![f, t, t, t, t, t, t, t, t, f],
                vec![t, f, t, f, t, t, t, f, t, f]
            ]),
            13
        )
    }
}
