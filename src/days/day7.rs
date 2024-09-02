use std::{
    collections::{HashMap, HashSet},
    usize,
};

use itertools::Itertools;

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let tower = Tower::parse(&lines)?;
        let part1 = tower.root.clone();

        Ok(DayResult {
            part1,
            part2: Some(tower.balance().to_string()),
        })
    }
}

struct Program {
    children: Vec<String>,
    weight: usize,
}

struct Tower {
    programs: HashMap<String, Program>,
    root: String,
}

impl Tower {
    fn parse(s: &Vec<String>) -> Result<Self, String> {
        let mut programs = HashMap::new();

        for l in s {
            let mut children = Vec::new();
            if l.contains(" -> ") {
                let s: Vec<_> = l.split(" -> ").collect();
                children = s[1].split(", ").map(|s| s.to_owned()).collect();
            }

            let s: Vec<_> = l.split(" ").collect();
            if s.len() < 2 {
                return Err(format!("bad format {}", l));
            }
            let name = s[0];
            let weight_raw = s[1];

            let brackets: &[_] = &['(', ')'];
            let weight = weight_raw
                .trim_matches(brackets)
                .parse::<usize>()
                .map_err(|e| format!("failed to parse weight {}: {}", weight_raw, e))?;
            programs.insert(name.to_owned(), Program { children, weight });
        }

        let mut children = HashSet::new();
        for p in programs.values() {
            children.extend(p.children.iter());
        }
        let root = programs
            .keys()
            .find(|p| !children.contains(p))
            .ok_or("could not find root".to_owned())?
            .to_owned();

        Ok(Tower { programs, root })
    }

    fn balance(&self) -> usize {
        let mut unbalance = usize::MAX;
        let mut correction = 0;
        for program in self.programs.values() {
            if program.children.len() > 2 {
                let counts = program.children.iter().map(|s| self.weight(s)).counts();
                if let Some((k, _)) = counts.iter().find(|(_, v)| **v == 1) {
                    let expected = *counts
                        .iter()
                        .find(|(_, v)| **v != 1)
                        .expect("must exist or cannot balance")
                        .0;
                    if expected < unbalance {
                        let p = program
                            .children
                            .iter()
                            .find(|s| self.weight(s) == *k)
                            .expect("already found");

                        let corrected = self.programs[p].weight + expected - k;

                        correction = corrected;
                        unbalance = expected;
                    }
                }
            }
        }
        correction
    }

    fn weight(&self, program: &String) -> usize {
        let mut total = 0;
        let p = &self.programs[program];

        for c in p.children.iter() {
            total += self.weight(c)
        }

        total + p.weight
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_parses() {
        let lines = vec![
            "pbga (66)".to_owned(),
            "xhth (57)".to_owned(),
            "ebii (61)".to_owned(),
            "havc (66)".to_owned(),
            "ktlj (57)".to_owned(),
            "fwft (72) -> ktlj, cntj, xhth".to_owned(),
            "qoyq (66)".to_owned(),
            "padx (45) -> pbga, havc, qoyq".to_owned(),
            "tknk (41) -> ugml, padx, fwft".to_owned(),
            "jptl (61)".to_owned(),
            "ugml (68) -> gyxo, ebii, jptl".to_owned(),
            "gyxo (61)".to_owned(),
            "cntj (57)".to_owned(),
        ];

        let t = Tower::parse(&lines).unwrap();
        assert_eq!(t.root, "tknk");
        assert_eq!(t.balance(), 60);
    }
}
