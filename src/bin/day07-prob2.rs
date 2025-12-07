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
    let mut prev: Vec<usize> = vec![0; splitters[0].len()];
    prev[start_i] = 1;
    let mut cur: Vec<usize> = vec![0; splitters[0].len()];

    for row in splitters.into_iter() {
        for (i, count) in prev.iter().enumerate() {
            if *count > 0 {
                if row[i] {
                    cur[i - 1] += count;
                    cur[i + 1] += count;
                } else {
                    cur[i] += *count;
                }
            }
        }
        // {
        //     for (i, count) in cur.iter().enumerate() {
        //         match *count {
        //             0 if row[i] => print!("^"),
        //             0 => print!("."),
        //             count if count < 10 => print!("{}", count),
        //             count if (10..16).contains(&count) => {
        //                 print!("{}", b'a' + (count - 10) as u8)
        //             }
        //             count => panic!("Unable to handle counts > 16 ({})", count),
        //         };
        //     }
        //     println!();
        // }
        std::mem::swap(&mut prev, &mut cur);
        cur.fill(0);
    }

    prev.into_iter().sum()
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
            40
        )
    }
}
