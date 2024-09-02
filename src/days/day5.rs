use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let part1 = Instructions::parse(&lines)?.steps(false).to_string();
        let part2 = Instructions::parse(&lines)?.steps(true).to_string();

        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}

struct Instructions(Vec<i32>);

impl Instructions {
    fn parse(lines: &Vec<String>) -> Result<Self, String> {
        let i = lines
            .iter()
            .map(|s| {
                s.parse::<i32>()
                    .map_err(|e| format!("failed to parse ({}): {}", s, e))
            })
            .collect::<Result<_, _>>()?;
        Ok(Instructions(i))
    }

    fn steps(&mut self, strange: bool) -> usize {
        let mut pointer = 0;
        let mut steps = 0;

        loop {
            if pointer >= self.0.len() {
                return steps;
            }
            steps += 1;
            let next = (pointer as i32) + self.0[pointer];
            if strange && self.0[pointer] >= 3 {
                self.0[pointer] -= 1;
            } else {
                self.0[pointer] += 1;
            }
            if next < 0 {
                return steps;
            }
            pointer = next as usize;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let mut i = Instructions(vec![0, 3, 0, 1, -3]);
        assert_eq!(i.steps(false), 5);

        let mut i = Instructions(vec![0, 3, 0, 1, -3]);
        assert_eq!(i.steps(true), 10);
    }
}
