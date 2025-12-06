use std::cmp::Ordering::{Equal, Greater, Less};
use std::cmp::max;

fn main() {
    let ims = std::fs::read_to_string("src/bin/day05.txt")
        .map(|file| parse_lines(file.lines().collect()))
        .expect("Unable to open file");
    println!("{}", find_fresh(ims))
}

#[derive(PartialEq, Debug)]
struct Ims {
    pub fresh: Vec<(u64, u64)>,
    pub available: Vec<u64>,
}

fn parse_lines(lines: Vec<&str>) -> Ims {
    let mut fresh: Vec<(u64, u64)> = Vec::new();
    let mut available: Vec<u64> = Vec::new();
    let mut in_fresh = true;

    for line in lines {
        if line.is_empty() {
            in_fresh = false;
        } else if in_fresh {
            let items: Vec<u64> = line
                .split("-")
                .map(|item| item.parse().expect("Unable to parse number"))
                .collect();
            fresh.push((items[0], items[1]));
        } else {
            let item: u64 = line.parse().expect("Unable to parse number");
            available.push(item);
        }
    }

    Ims { fresh, available }
}

/// take the list of ranges supplied and merge all overlapping ranges, returning a minimal list of
/// ranges, sorted by starting value to enable efficient binary searches
fn condense_ranges(ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let sorted_ranges = {
        let mut temp = ranges.clone();
        temp.sort();
        temp
    };
    let mut condensed: Vec<(u64, u64)> = Vec::with_capacity(ranges.len());

    for range in sorted_ranges.into_iter() {
        if condensed.is_empty() {
            condensed.push(range);
        } else {
            let last_i = condensed.len() - 1;
            if condensed[last_i].1 < range.0 - 1 {
                condensed.push(range);
            } else {
                condensed[last_i].1 = max(range.1, condensed[last_i].1);
            }
        }
    }

    println!(
        "Orig ranges: {}, condensed ranges: {}",
        ranges.len(),
        condensed.len()
    );
    condensed
}

// fn find_fresh(ims: Ims) -> usize {
//     let ranges = condense_ranges(ims.fresh);
//     let available = {
//         let mut temp = ims.available.clone();
//         temp.sort();
//         temp
//     };
//
//     let mut av_i: usize = 0;
//     let mut count: usize = 0;
//     for (range_min, range_max) in ranges.into_iter() {
//         while av_i < available.len() && available[av_i] <= range_max {
//             if range_min <= available[av_i] && available[av_i] <= range_max {
//                 count += 1;
//             }
//             av_i += 1;
//         }
//         av_i -= 1;
//     }
//
//     count
// }

fn find_fresh(ims: Ims) -> usize {
    let ranges = condense_ranges(ims.fresh);
    ims.available
        .into_iter()
        .filter(|item| {
            ranges
                .binary_search_by(|(start, end)| {
                    if start <= item && item <= end {
                        Equal
                    } else if item < start {
                        Greater
                    } else {
                        Less
                    }
                })
                .is_ok()
        })
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_lines() {
        assert_eq!(
            parse_lines(vec![
                "3-5", "10-14", "16-20", "12-18", "", "1", "5", "8", "11", "17", "32",
            ]),
            Ims {
                fresh: vec![(3, 5), (10, 14), (16, 20), (12, 18)],
                available: vec![1, 5, 8, 11, 17, 32]
            }
        )
    }

    #[test]
    fn test_condense_ranges() {
        assert_eq!(
            condense_ranges(vec![(3, 5), (10, 14), (16, 20), (12, 18)]),
            vec![(3, 5), (10, 20)]
        )
    }

    #[test]
    fn test_condense_ranges_neighbors() {
        assert_eq!(condense_ranges(vec![(3, 5), (6, 10)]), vec![(3, 10)])
    }

    #[test]
    fn test_condense_ranges_nested() {
        assert_eq!(condense_ranges(vec![(3, 10), (4, 7)]), vec![(3, 10)])
    }

    #[test]
    fn test_find_fresh() {
        assert_eq!(
            find_fresh(Ims {
                fresh: vec![(3, 5), (10, 14), (16, 20), (12, 18)],
                available: vec![1, 5, 8, 11, 17, 32]
            }),
            3
        );
    }
}
