fn main() {
    let mut banks: Vec<Vec<bool>> = std::fs::read_to_string("src/bin/day04.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(parse_line)
                .collect()
        })
        .expect("Unable to open file");
    println!("{}", remove_rolls(&mut banks))
}

fn parse_line(line: &str) -> Vec<bool> {
    line.chars().map(|char| char == '@').collect()
}

fn remove_rolls(map: &mut [Vec<bool>]) -> usize {
    let mut removed_count: usize = 0;
    loop {
        let accessible_coords = find_accessible_rolls(map);
        for coord in accessible_coords.iter() {
            map[coord.0][coord.1] = false;
        }
        println!("Removed {} rolls", accessible_coords.len());
        removed_count += accessible_coords.len();
        if accessible_coords.is_empty() {
            break;
        }
    }
    removed_count
}

fn find_accessible_rolls(map: &[Vec<bool>]) -> Vec<(usize, usize)> {
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

    let mut removable: Vec<(usize, usize)> = Vec::new();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] && neighbor_count[i][j] < 4 {
                removable.push((i, j));
            }
        }
    }
    removable
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
    fn test_find_accessible_rolls() {
        let f = false;
        let t = true;
        assert_eq!(
            find_accessible_rolls(&vec![
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
            vec![
                (0, 2),
                (0, 3),
                (0, 5),
                (0, 6),
                (0, 8),
                (1, 0),
                (2, 6),
                (4, 0),
                (4, 9),
                (7, 0),
                (9, 0),
                (9, 2),
                (9, 8)
            ]
        )
    }

    #[test]
    fn test_remove_rolls() {
        let f = false;
        let t = true;
        assert_eq!(
            remove_rolls(&mut vec![
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
            43
        )
    }
}
