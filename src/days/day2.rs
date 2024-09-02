use itertools::Itertools;

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let parsed = parse(lines)?;
        let part1 = checksum(&parsed).to_string();
        let part2 = checksum_divisible(&parsed).to_string();

        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}

fn parse(lines: Vec<String>) -> Result<Vec<Vec<u32>>, String> {
    lines
        .iter()
        .map(|l| {
            l.split_whitespace()
                .filter(|s| !s.is_empty())
                .map(|s| {
                    s.parse::<u32>()
                        .map_err(|e| format!("parse failure ({}): {}", s, e))
                })
                .collect()
        })
        .collect()
}

fn checksum(nums: &Vec<Vec<u32>>) -> u32 {
    nums.into_iter()
        .map(|l| l.iter().max().unwrap_or(&0) - l.iter().min().to_owned().unwrap_or(&0))
        .sum()
}

fn checksum_divisible(nums: &Vec<Vec<u32>>) -> u32 {
    nums.into_iter()
        .map(|l| {
            l.into_iter()
                .tuple_combinations::<(_, _)>()
                .map(|(a, b)| (*a.max(b), *a.min(b)))
                .find(|(a, b)| a % b == 0)
                .map_or(0, |(a, b)| a / b)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_numbers() {
        assert_eq!(
            parse(vec!["1  2 3  4".to_string(), "5 6 7".to_string()]).unwrap(),
            vec![vec![1, 2, 3, 4], vec![5, 6, 7]]
        )
    }

    #[test]
    fn it_gives_checksum() {
        let example = vec![vec![5, 1, 9, 5], vec![7, 5, 3], vec![2, 4, 6, 8]];
        assert_eq!(checksum(&example), 18)
    }

    #[test]
    fn it_gives_checksum_divisible() {
        let example = vec![vec![5, 9, 2, 8], vec![9, 4, 7, 3], vec![3, 8, 6, 5]];
        assert_eq!(checksum_divisible(&example), 9)
    }
}
