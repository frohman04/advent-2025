fn main() {
    let data = std::fs::read_to_string("src/bin/day12.txt")
        .map(|file| parse_lines(file.lines().collect()))
        .expect("Unable to open file");
    println!("{}", sanity_check(data))
}

fn parse_lines(lines: Vec<&str>) -> Vec<((usize, usize), Vec<usize>)> {
    lines
        .into_iter()
        .filter_map(|line| {
            if line.contains("x") {
                let first_split: Vec<&str> = line.split(": ").collect();
                let dims: Vec<usize> = first_split[0]
                    .split("x")
                    .map(|dim| dim.parse().expect("unable to parse dimension"))
                    .collect();
                let counts: Vec<usize> = first_split[1]
                    .split(" ")
                    .map(|count| count.parse().expect("unable to parse count"))
                    .collect();
                Some(((dims[0], dims[1]), counts))
            } else {
                None
            }
        })
        .collect()
}

fn sanity_check(data: Vec<((usize, usize), Vec<usize>)>) -> usize {
    data.into_iter()
        .filter(|((x, y), counts)| counts.iter().sum::<usize>() * 9 <= x * y)
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_lines() {
        assert_eq!(
            parse_lines(vec![
                "0:",
                "###",
                "##.",
                "##.",
                "",
                "1:",
                "###",
                "##.",
                ".##",
                "",
                "2:",
                ".##",
                "###",
                "##.",
                "",
                "3:",
                "##.",
                "###",
                "##.",
                "",
                "4:",
                "###",
                "#..",
                "###",
                "",
                "5:",
                "###",
                ".#.",
                "###",
                "",
                "4x4: 0 0 0 0 2 0",
                "12x5: 1 0 1 0 2 2",
                "12x5: 1 0 1 0 3 2",
            ]),
            vec![
                ((4, 4), vec![0, 0, 0, 0, 2, 0]),
                ((12, 5), vec![1, 0, 1, 0, 2, 2]),
                ((12, 5), vec![1, 0, 1, 0, 3, 2]),
            ]
        )
    }
}
