use std::str::FromStr;

use super::day::*;

pub struct Instance;

const TOTAL_DANCE: usize = 1_000_000_000;
const TOTAL_PROGRAMS: u8 = 16;

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let line = lines.first().ok_or("empty lines".to_string())?;

        let moves = line
            .split(",")
            .map(|s| s.parse())
            .collect::<Result<Vec<Move>, _>>()?;

        let mut dance = Dance::new(TOTAL_PROGRAMS);
        dance.dance(moves.as_slice());
        let part1 = dance.to_string();

        let initial = Dance::new(TOTAL_PROGRAMS).to_string();

        for i in 2..TOTAL_DANCE {
            dance.dance(moves.as_slice());
            if dance.to_string() == initial {
                let cycle = i;
                let remaining = TOTAL_DANCE % cycle;
                for _ in 0..remaining {
                    dance.dance(moves.as_slice());
                }
                break;
            }
        }
        Ok(DayResult {
            part1,
            part2: Some(dance.to_string()),
        })
    }
}

enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let kind = chars.next().ok_or("empty string")?;
        let rest = chars.as_str();
        match kind {
            's' => rest
                .parse()
                .map(Move::Spin)
                .map_err(|e| format!("failed to parse spin: {}", e)),
            'x' => {
                let mut parts = rest.split("/");
                let a = parts
                    .next()
                    .ok_or("missing first part")?
                    .parse()
                    .map_err(|e| format!("failed to parse first part: {}", e))?;
                let b = parts
                    .next()
                    .ok_or("missing second part")?
                    .parse()
                    .map_err(|e| format!("failed to parse second part: {}", e))?;
                Ok(Move::Exchange(a, b))
            }
            'p' => {
                let mut parts = rest.split("/");
                let a = parts
                    .next()
                    .ok_or("missing first part")?
                    .parse()
                    .map_err(|e| format!("failed to parse first part: {}", e))?;
                let b = parts
                    .next()
                    .ok_or("missing second part")?
                    .parse()
                    .map_err(|e| format!("failed to parse second part: {}", e))?;
                Ok(Move::Partner(a, b))
            }
            _ => Err(format!("unknown move kind: {}", kind)),
        }
    }
}

struct Dance {
    programs: Vec<char>,
}

impl Dance {
    fn new(size: u8) -> Self {
        Dance {
            programs: (b'a'..(b'a' + size)).map(char::from).collect(),
        }
    }

    fn spin(&mut self, count: usize) {
        let count = self.programs.len() - count;
        let mut new = self.programs[count..].to_vec();
        new.extend_from_slice(&self.programs[..count]);
        self.programs = new;
    }

    fn exchange(&mut self, a: usize, b: usize) {
        self.programs.swap(a, b);
    }

    fn partner(&mut self, a: char, b: char) {
        let a = self.programs.iter().position(|&c| c == a).unwrap();
        let b = self.programs.iter().position(|&c| c == b).unwrap();
        self.exchange(a, b);
    }

    fn dance(&mut self, moves: &[Move]) {
        for m in moves {
            match m {
                Move::Spin(count) => self.spin(*count),
                Move::Exchange(a, b) => self.exchange(*a, *b),
                Move::Partner(a, b) => self.partner(*a, *b),
            }
        }
    }
}

impl ToString for Dance {
    fn to_string(&self) -> String {
        self.programs.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dance() {
        let mut dance = Dance::new(5);
        dance.dance(&[Move::Spin(1), Move::Exchange(3, 4), Move::Partner('e', 'b')]);
        assert_eq!(dance.to_string(), "baedc");
    }
}
