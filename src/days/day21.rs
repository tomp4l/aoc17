use std::{str::FromStr, vec};

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let rule_book = RuleBook::new(
            lines
                .iter()
                .map(|line| line.parse())
                .collect::<Result<Vec<Rule>, _>>()?,
        );

        let part1 = simulate(&rule_book, 5).to_string();
        let part2 = simulate(&rule_book, 18).to_string();
        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Rule {
    size: usize,
    to: Vec<Vec<bool>>,
    permutations: Vec<Vec<Vec<bool>>>,
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" => ");
        let from = parts.next().ok_or("missing from")?;
        let to = parts.next().ok_or("missing to")?;

        let from = from
            .split("/")
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();
        let to = to
            .split("/")
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();

        Ok(Rule::new(from, to))
    }
}

impl Rule {
    fn new(mut from: Vec<Vec<bool>>, to: Vec<Vec<bool>>) -> Rule {
        let mut permutations = vec![];
        for _ in 0..4 {
            flip_ver(&mut from);
            permutations.push(from.clone());
            flip_ver(&mut from);
            permutations.push(from.clone());
            rotate(&mut from);
            permutations.push(from.clone());
        }

        permutations.sort();
        permutations.dedup();
        Rule {
            size: from.len(),
            to,
            permutations,
        }
    }

    fn apply(&self, portion: &Vec<Vec<bool>>) -> Option<Vec<Vec<bool>>> {
        let to = &self.to;

        for permutation in &self.permutations {
            if permutation == portion {
                return Some(to.clone());
            }
        }

        None
    }
}

fn flip_ver(portion: &mut Vec<Vec<bool>>) {
    portion.reverse();
}

fn rotate(portion: &mut Vec<Vec<bool>>) {
    let mut rotated = vec![vec![false; portion.len()]; portion.len()];
    for (y, row) in portion.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            rotated[x][portion.len() - y - 1] = value;
        }
    }

    *portion = rotated;
}

struct RuleBook {
    two_rules: Vec<Rule>,
    three_rules: Vec<Rule>,
}

impl RuleBook {
    fn new(rules: Vec<Rule>) -> RuleBook {
        let (two_rules, three_rules) = rules.into_iter().partition(|rule| rule.size == 2);
        RuleBook {
            two_rules,
            three_rules,
        }
    }

    fn apply(&self, grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
        let size = if grid.len() % 2 == 0 { 2 } else { 3 };
        let rules = if grid.len() % 2 == 0 {
            &self.two_rules
        } else {
            &self.three_rules
        };
        let new_size = size + 1;
        let mut new_grid =
            vec![vec![false; grid.len() / size * new_size]; grid.len() / size * new_size];

        for y in 0..grid.len() / size {
            for x in 0..grid.len() / size {
                let mut portion = vec![vec![false; size]; size];
                for dy in 0..size {
                    for dx in 0..size {
                        portion[dy][dx] = grid[y * size + dy][x * size + dx];
                    }
                }

                for rule in rules {
                    if let Some(to) = rule.apply(&portion) {
                        for dy in 0..new_size {
                            for dx in 0..new_size {
                                new_grid[y * new_size + dy][x * new_size + dx] = to[dy][dx];
                            }
                        }
                    }
                }
            }
        }

        new_grid
    }
}

fn simulate(rule_book: &RuleBook, iterations: usize) -> usize {
    let mut grid = vec![
        vec![false, true, false],
        vec![false, false, true],
        vec![true, true, true],
    ];

    for _ in 0..iterations {
        grid = rule_book.apply(&grid);
    }

    grid.iter().flatten().filter(|&&v| v).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let rules: Vec<Rule> = vec![
            "../.# => ##./#../...".parse().unwrap(),
            ".#./..#/### => #..#/..../..../#..#".parse().unwrap(),
        ];

        let rule_book = RuleBook::new(rules);
        assert_eq!(simulate(&rule_book, 2), 12);
    }
}
