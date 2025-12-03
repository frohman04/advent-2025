fn main() {
    let cmds = std::fs::read_to_string("src/bin/day01.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(Cmd::from_line)
                .collect::<Vec<Cmd>>()
        })
        .expect("Unable to open file");
    let mut dial = Dial::default();
    cmds.into_iter().for_each(|cmd| dial.spin(cmd));
    println!("{:?}", dial);
}

#[derive(Debug, PartialEq)]
enum Cmd {
    Left(u16),
    Right(u16),
}

impl Cmd {
    pub fn from_line(line: &str) -> Cmd {
        let chars: Vec<char> = line.chars().collect();
        let dir = chars[0];
        let amt = chars[1..]
            .iter()
            .collect::<String>()
            .parse::<u16>()
            .expect("unable to parse");

        match dir {
            'L' => Cmd::Left(amt),
            'R' => Cmd::Right(amt),
            x => panic!("Unknown direction: {}", x),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Dial {
    pos: u16,
    times_on_zero: u16,
}

impl Default for Dial {
    fn default() -> Self {
        Dial {
            pos: 50,
            times_on_zero: 0,
        }
    }
}

impl Dial {
    pub fn spin(&mut self, cmd: Cmd) {
        let max: u32 = 100;

        self.pos = match cmd {
            Cmd::Left(amt) => {
                ((self.pos as u32 + (max * (amt as u32 / max + 1)) - amt as u32) % max) as u16
            }
            Cmd::Right(amt) => ((self.pos as u32 + amt as u32) % max) as u16,
        };

        if self.pos == 0 {
            self.times_on_zero += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_line_left() {
        assert_eq!(Cmd::from_line("L50"), Cmd::Left(50))
    }

    #[test]
    fn test_from_line_right() {
        assert_eq!(Cmd::from_line("R9"), Cmd::Right(9))
    }

    #[test]
    fn test_spin_right() {
        let mut dial = Dial {
            pos: 11,
            times_on_zero: 0,
        };
        dial.spin(Cmd::Right(8));
        assert_eq!(
            dial,
            Dial {
                pos: 19,
                times_on_zero: 0
            }
        );
    }

    #[test]
    fn test_spin_left() {
        let mut dial = Dial {
            pos: 19,
            times_on_zero: 0,
        };
        dial.spin(Cmd::Left(19));
        assert_eq!(
            dial,
            Dial {
                pos: 0,
                times_on_zero: 1
            }
        );
    }

    #[test]
    fn test_spin_wrap_left() {
        let mut dial = Dial {
            pos: 0,
            times_on_zero: 0,
        };
        dial.spin(Cmd::Left(1));
        assert_eq!(
            dial,
            Dial {
                pos: 99,
                times_on_zero: 0
            }
        );
    }

    #[test]
    fn test_spin_wrap_right() {
        let mut dial = Dial {
            pos: 99,
            times_on_zero: 0,
        };
        dial.spin(Cmd::Right(1));
        assert_eq!(
            dial,
            Dial {
                pos: 0,
                times_on_zero: 1
            }
        );
    }

    #[test]
    fn test_spin_multiple_wrap_left() {
        let mut dial = Dial::default();
        dial.spin(Cmd::Left(199));
        assert_eq!(
            dial,
            Dial {
                pos: 51,
                times_on_zero: 0
            }
        );
    }

    #[test]
    fn test_spin_multiple_wrap_right() {
        let mut dial = Dial::default();
        dial.spin(Cmd::Right(199));
        assert_eq!(
            dial,
            Dial {
                pos: 49,
                times_on_zero: 0
            }
        );
    }

    #[test]
    fn test_full() {
        let cmds = vec![
            Cmd::Left(68),
            Cmd::Left(30),
            Cmd::Right(48),
            Cmd::Left(5),
            Cmd::Right(60),
            Cmd::Left(55),
            Cmd::Left(1),
            Cmd::Left(99),
            Cmd::Right(14),
            Cmd::Left(82),
        ];
        let mut dial = Dial::default();
        cmds.into_iter().for_each(|cmd| dial.spin(cmd));
        assert_eq!(
            dial,
            Dial {
                pos: 32,
                times_on_zero: 3
            }
        )
    }
}
