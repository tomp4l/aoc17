use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let numbers = parse(lines)?;
        let part1 = add_if_match(&numbers, 1).to_string();
        let part2 = add_if_match(&numbers, numbers.len() / 2).to_string();

        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}

fn parse(lines: Vec<String>) -> Result<Vec<u32>, String> {
    lines
        .first()
        .ok_or("empty lines".to_string())?
        .chars()
        .map(|d| d.to_digit(10).ok_or(format!("bad digit {}", d)))
        .collect::<Result<Vec<_>, _>>()
}

fn add_if_match(numbers: &Vec<u32>, offset: usize) -> u32 {
    (numbers)
        .iter()
        .enumerate()
        .filter(|(a, b)| numbers[(*a + offset) % numbers.len()] == **b)
        .map(|t| t.1)
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_numbers() {
        let lines = vec!["1234".to_string()];
        assert_eq!(parse(lines).unwrap(), vec![1, 2, 3, 4])
    }

    #[test]
    fn examples() {
        let examples = vec![("1122", 3), ("1111", 4), ("1234", 0), ("91212129", 9)];

        for e in examples.into_iter() {
            let p = parse(vec![e.0.to_string()]).unwrap();
            assert_eq!(add_if_match(&p, 1), e.1)
        }

        let examples = vec![
            ("1212", 6),
            ("1221", 0),
            ("123425", 4),
            ("123123", 12),
            ("12131415", 4),
        ];

        for e in examples.into_iter() {
            let p = parse(vec![e.0.to_string()]).unwrap();
            assert_eq!(add_if_match(&p, p.len() / 2), e.1)
        }
    }
}
