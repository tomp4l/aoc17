use super::day::*;

pub struct Instance;

const YEAR: usize = 2017;

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let step = lines
            .first()
            .ok_or("empty lines".to_string())?
            .parse::<usize>()
            .map_err(|e| e.to_string())?;

        Ok(DayResult {
            part1: part1(step).to_string(),
            part2: Some(part2(step).to_string()),
        })
    }
}

#[derive(Debug)]
struct Spinlock {
    buffer: Vec<usize>,
    position: usize,
    step: usize,
}

impl Spinlock {
    fn new(step: usize) -> Spinlock {
        Spinlock {
            buffer: vec![0],
            position: 0,
            step,
        }
    }

    fn insert(&mut self, value: usize) {
        self.position = (self.position + self.step) % self.buffer.len() + 1;
        self.buffer.insert(self.position, value);
    }

    fn value_after(&self, value: usize) -> usize {
        let position = self.buffer.iter().position(|&v| v == value).unwrap();
        self.buffer[(position + 1) % self.buffer.len()]
    }
}

fn part1(step: usize) -> usize {
    let mut spinlock = Spinlock::new(step);
    for i in 1..=YEAR {
        spinlock.insert(i);
    }
    spinlock.value_after(YEAR)
}

fn part2(step: usize) -> usize {
    let mut position = 0;
    let mut value = 0;
    for i in 1..50_000_000 {
        position = (position + step) % i + 1;
        if position == 1 {
            value = i;
        }
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(3), 638);
        assert_eq!(part2(3), 1222153);
    }
}
