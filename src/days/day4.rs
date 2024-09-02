use itertools::Itertools;

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let part1 = lines.iter().filter(|s| is_valid(s)).count().to_string();
        let part2 = lines
            .iter()
            .filter(|s| is_valid_anagram(s))
            .count()
            .to_string();

        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}

fn is_valid(phrase: &String) -> bool {
    let words: Vec<_> = phrase.split(" ").collect();
    let unique = words.iter().unique();
    words.len() == unique.count()
}

fn is_valid_anagram(phrase: &String) -> bool {
    let words: Vec<_> = phrase.split(" ").collect();
    let unique = words
        .iter()
        .map(|word| word.chars().sorted().collect::<String>())
        .unique();
    words.len() == unique.count()
}

#[cfg(test)]
mod tests {
    use crate::day4::{is_valid, is_valid_anagram};

    #[test]
    fn test_valid() {
        assert!(is_valid(&"aa bb cc dd ee".to_owned()));
        assert!(!is_valid(&"aa bb cc dd aa".to_owned()));
        assert!(is_valid(&"aa bb cc dd aaa".to_owned()));
    }

    #[test]
    fn test_valid_anagram() {
        assert!(is_valid_anagram(&"abcde fghij".to_owned()));
        assert!(!is_valid_anagram(&"abcde xyz ecdab".to_owned()));
        assert!(is_valid_anagram(&"a ab abc abd abf abj".to_owned()));
        assert!(is_valid_anagram(&"iiii oiii ooii oooi oooo".to_owned()));
        assert!(!is_valid_anagram(&"oiii ioii iioi iiio".to_owned()));
    }
}
