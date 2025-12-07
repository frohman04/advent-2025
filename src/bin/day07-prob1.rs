use std::collections::HashSet;

fn main() {
    let (start_i, splitters) = std::fs::read_to_string("src/bin/day07.txt")
        .map(|file| parse_lines(file.lines().collect()))
        .expect("Unable to open file");
    println!("{}", propagate_beam(start_i, splitters))
}

fn parse_lines(lines: Vec<&str>) -> (usize, Vec<Vec<bool>>) {
    let start_i: usize = lines[0]
        .chars()
        .enumerate()
        .filter_map(|(i, char)| if char == 'S' { Some(i) } else { None })
        .next()
        .expect("Unable to find starting position");

    let splitters: Vec<Vec<bool>> = lines[1..]
        .iter()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '.' => false,
                    '^' => true,
                    x => panic!("Unexpected character detected: {}", x),
                })
                .collect()
        })
        .collect();

    (start_i, splitters)
}

fn propagate_beam(start_i: usize, splitters: Vec<Vec<bool>>) -> usize {
    let mut prev: HashSet<usize> = HashSet::new();
    prev.insert(start_i);
    let mut cur: HashSet<usize> = HashSet::new();
    let mut count: usize = 0;

    for row in splitters.into_iter() {
        for beam in prev.iter() {
            if row[*beam] {
                cur.insert(beam - 1);
                cur.insert(beam + 1);
                count += 1;
            } else {
                cur.insert(*beam);
            }
        }
        std::mem::swap(&mut prev, &mut cur);
        cur.clear();
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_lines() {
        let t = true;
        let f = false;

        assert_eq!(
            parse_lines(vec![
                ".......S.......",
                "...............",
                ".......^.......",
                "...............",
                "......^.^......",
                "...............",
                ".....^.^.^.....",
                "...............",
                "....^.^...^....",
                "...............",
                "...^.^...^.^...",
                "...............",
                "..^...^.....^..",
                "...............",
                ".^.^.^.^.^...^.",
                "...............",
            ]),
            (
                7,
                vec![
                    vec![f, f, f, f, f, f, f, f, f, f, f, f, f, f, f],
                    vec![f, f, f, f, f, f, f, t, f, f, f, f, f, f, f],
                    vec![f, f, f, f, f, f, f, f, f, f, f, f, f, f, f],
                    vec![f, f, f, f, f, f, t, f, t, f, f, f, f, f, f],
                    vec![f, f, f, f, f, f, f, f, f, f, f, f, f, f, f],
                    vec![f, f, f, f, f, t, f, t, f, t, f, f, f, f, f],
                    vec![f, f, f, f, f, f, f, f, f, f, f, f, f, f, f],
                    vec![f, f, f, f, t, f, t, f, f, f, t, f, f, f, f],
                    vec![f, f, f, f, f, f, f, f, f, f, f, f, f, f, f],
                    vec![f, f, f, t, f, t, f, f, f, t, f, t, f, f, f],
                    vec![f, f, f, f, f, f, f, f, f, f, f, f, f, f, f],
                    vec![f, f, t, f, f, f, t, f, f, f, f, f, t, f, f],
                    vec![f, f, f, f, f, f, f, f, f, f, f, f, f, f, f],
                    vec![f, t, f, t, f, t, f, t, f, t, f, f, f, t, f],
                    vec![f, f, f, f, f, f, f, f, f, f, f, f, f, f, f],
                ]
            )
        )
    }

    #[test]
    fn test_propagate_beam() {
        let t = true;
        let f = false;

        assert_eq!(
            propagate_beam(
                7,
                vec![
                    vec![f, f, f, f, f, f, f, f, f, f, f, f, f, f, f],
                    vec![f, f, f, f, f, f, f, t, f, f, f, f, f, f, f],
                    vec![f, f, f, f, f, f, f, f, f, f, f, f, f, f, f],
                    vec![f, f, f, f, f, f, t, f, t, f, f, f, f, f, f],
                    vec![f, f, f, f, f, f, f, f, f, f, f, f, f, f, f],
                    vec![f, f, f, f, f, t, f, t, f, t, f, f, f, f, f],
                    vec![f, f, f, f, f, f, f, f, f, f, f, f, f, f, f],
                    vec![f, f, f, f, t, f, t, f, f, f, t, f, f, f, f],
                    vec![f, f, f, f, f, f, f, f, f, f, f, f, f, f, f],
                    vec![f, f, f, t, f, t, f, f, f, t, f, t, f, f, f],
                    vec![f, f, f, f, f, f, f, f, f, f, f, f, f, f, f],
                    vec![f, f, t, f, f, f, t, f, f, f, f, f, t, f, f],
                    vec![f, f, f, f, f, f, f, f, f, f, f, f, f, f, f],
                    vec![f, t, f, t, f, t, f, t, f, t, f, f, f, t, f],
                    vec![f, f, f, f, f, f, f, f, f, f, f, f, f, f, f],
                ]
            ),
            21
        )
    }
}
