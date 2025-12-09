use std::cmp::PartialOrd;

fn main() {
    let tiles: Vec<Pos> = std::fs::read_to_string("src/bin/day09.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(parse_line)
                .collect()
        })
        .expect("Unable to open file");
    println!("{}", calc_sizes(tiles))
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
struct Pos {
    pub x: u32,
    pub y: u32,
}

fn parse_line(line: &str) -> Pos {
    let parts: Vec<u32> = line
        .split(",")
        .map(|num| num.parse().expect("unable to parse number"))
        .collect();
    Pos {
        x: parts[0],
        y: parts[1],
    }
}

fn size(pos1: &Pos, pos2: &Pos) -> u64 {
    let x = pos1.x.abs_diff(pos2.x);
    let y = pos1.y.abs_diff(pos2.y);
    (x + 1) as u64 * (y + 1) as u64
}

fn calc_sizes(tiles: Vec<Pos>) -> u64 {
    let sorted_tiles = {
        let mut temp: Vec<Pos> = tiles.into_iter().collect();
        temp.sort();
        temp
    };
    let mut max: Option<u64> = None;
    for (i, tile1) in sorted_tiles.iter().enumerate() {
        for tile2 in sorted_tiles[i + 1..].iter() {
            let s = size(tile1, tile2);
            // println!("{:?} {:?} {}", tile1, tile2, s);
            if s > max.unwrap_or(0) {
                max = Some(s);
            }
        }
    }

    max.expect("No areas computed!")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("7,1"), Pos { x: 7, y: 1 })
    }

    #[test]
    fn test_calc_sizes() {
        assert_eq!(
            calc_sizes(vec![
                Pos { x: 7, y: 1 },
                Pos { x: 11, y: 1 },
                Pos { x: 11, y: 7 },
                Pos { x: 9, y: 7 },
                Pos { x: 9, y: 5 },
                Pos { x: 2, y: 5 },
                Pos { x: 2, y: 3 },
                Pos { x: 7, y: 3 },
            ]),
            50
        )
    }
}
