use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let pipes = parse_input(&lines)?;

        let part1 = pipes.group(0).len().to_string();
        let part2 = pipes.groups().to_string();
        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}

fn parse_input(input: &[String]) -> Result<Pipes, String> {
    let mut pipes = Pipes::new();
    for line in input {
        let parts = line.split(" <-> ").collect_vec();
        if parts.len() != 2 {
            return Err(format!("invalid line: {}", line));
        }
        let from = parts[0]
            .parse::<usize>()
            .map_err(|e| format!("number parse failure {}: {}", line, e))?;
        let tos: Vec<_> = parts[1]
            .split(", ")
            .map(|s| s.parse())
            .collect::<Result<_, _>>()
            .map_err(|e| format!("number parse failure {}: {}", line, e))?;
        for to in tos {
            pipes.add_pipe(from, to);
        }
    }
    Ok(pipes)
}

struct Pipes {
    pipes: HashMap<usize, Vec<usize>>,
}

impl Pipes {
    fn new() -> Pipes {
        Pipes {
            pipes: HashMap::new(),
        }
    }

    fn add_pipe(&mut self, from: usize, to: usize) {
        self.pipes.entry(from).or_insert_with(Vec::new).push(to);
    }

    fn group(&self, start: usize) -> HashSet<usize> {
        let mut search = vec![start];
        let mut seen = HashSet::new();
        seen.insert(start);
        while let Some(current) = search.pop() {
            for &next in self.pipes.get(&current).unwrap_or(&Vec::new()) {
                if !seen.contains(&next) {
                    search.push(next);
                    seen.insert(next);
                }
            }
        }
        seen
    }

    fn groups(&self) -> usize {
        let mut groups = 0;
        let mut seen = HashSet::new();
        for &from in self.pipes.keys() {
            if !seen.contains(&from) {
                groups += 1;
                seen.extend(self.group(from));
            }
        }
        groups
    }
}
