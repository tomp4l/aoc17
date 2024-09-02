use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let generators = parse(&lines)?;
        Ok(DayResult {
            part1: part1(generators.clone()).to_string(),
            part2: Some(part2(generators.clone()).to_string()),
        })
    }
}

fn parse(input: &[String]) -> Result<(Generator, Generator), String> {
    if input.len() != 2 {
        return Err("expected two lines".to_string());
    }

    let pattern = "starts with ";
    let a = input[0]
        .split(pattern)
        .nth(1)
        .ok_or("no number in line 1")?
        .parse::<u64>()
        .map_err(|e| format!("failed to parse number in line 1: {}", e))?;

    let b = input[1]
        .split(pattern)
        .nth(1)
        .ok_or("no number in line 2")?
        .parse::<u64>()
        .map_err(|e| format!("failed to parse number in line 2: {}", e))?;

    Ok((Generator::new_a(a), Generator::new_b(b)))
}

fn part1(mut generators: (Generator, Generator)) -> usize {
    let mut count = 0;
    for _ in 0..40_000_000 {
        let (a, b) = (generators.0.next(), generators.1.next());
        if a as u16 == b as u16 {
            count += 1;
        }
    }
    count
}

fn part2(mut generators: (Generator, Generator)) -> usize {
    let mut count = 0;
    for _ in 0..5_000_000 {
        let (a, b) = (generators.0.next_filtered(), generators.1.next_filtered());
        if a as u16 == b as u16 {
            count += 1;
        }
    }
    count
}

#[derive(Debug, Clone)]
struct Generator {
    seed: u64,
    mult: u64,
    filter: u64,
}

impl Generator {
    fn new_a(seed: u64) -> Generator {
        Generator {
            seed,
            mult: 16807,
            filter: 4,
        }
    }

    fn new_b(seed: u64) -> Generator {
        Generator {
            seed,
            mult: 48271,
            filter: 8,
        }
    }

    fn next(&mut self) -> u64 {
        self.seed = (self.seed * self.mult) % 2147483647;
        self.seed
    }

    fn next_filtered(&mut self) -> u64 {
        loop {
            let next = self.next();
            if next % self.filter == 0 {
                return next;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = vec![
            "Generator A starts with 65".to_string(),
            "Generator B starts with 8921".to_string(),
        ];
        let generators = parse(&input).unwrap();
        assert_eq!(part1(generators.clone()), 588);
        assert_eq!(part2(generators), 309);
    }
}
