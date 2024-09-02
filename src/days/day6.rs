use std::collections::HashMap;

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let line = lines.first().ok_or("expected line".to_owned())?;
        let (p1, p2) = Memory::parse(line)?.redistribute_cycle();

        Ok(DayResult {
            part1: p1.to_string(),
            part2: Some(p2.to_string()),
        })
    }
}

struct Memory(Vec<u8>);

impl Memory {
    fn parse(line: &str) -> Result<Self, String> {
        let i = line
            .split_whitespace()
            .map(|s| {
                s.parse::<u8>()
                    .map_err(|e| format!("failed to parse ({}): {}", s, e))
            })
            .collect::<Result<_, _>>()?;
        Ok(Memory(i))
    }

    fn redistribute_cycle(&mut self) -> (usize, usize) {
        let mut seen_states = HashMap::new();
        let mut cycles = 0;

        loop {
            cycles += 1;
            self.redistribute();
            if let Some(last_seen) = seen_states.get(&self.0) {
                return (cycles, cycles - last_seen);
            }
            seen_states.insert(self.0.clone(), cycles);
        }
    }

    fn redistribute(&mut self) {
        let mut max = 0;
        let mut max_index = 0;
        for i in 0..self.0.len() {
            if self.0[i] > max {
                max = self.0[i];
                max_index = i;
            }
        }

        let mut to_distribute = self.0[max_index];
        self.0[max_index] = 0;
        let mut current_index = max_index;

        while to_distribute > 0 {
            current_index += 1;
            if current_index == self.0.len() {
                current_index = 0;
            }
            to_distribute -= 1;
            self.0[current_index] += 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(Memory(vec![0, 2, 7, 0]).redistribute_cycle(), (5, 4))
    }
}
