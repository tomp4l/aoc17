use std::str::FromStr;

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let thing: Thing = lines.first().ok_or("empty lines".to_string())?.parse()?;

        let part1 = thing.total_score().to_string();
        Ok(DayResult {
            part1,
            part2: Some(thing.total_garbage().to_string()),
        })
    }
}

enum Thing {
    Group { children: Vec<Thing>, score: usize },
    Garbage { content: String },
}

impl FromStr for Thing {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse(s: &str, i: usize, score: usize) -> Result<(Thing, usize), String> {
            match &s[i..i + 1] {
                "{" => {
                    let mut j = i + 1;
                    let mut children = Vec::new();
                    loop {
                        if &s[j..j + 1] == "}" {
                            return Ok((Thing::Group { children, score }, j + 1));
                        } else {
                            let (child, o) = parse(s, j, score + 1)?;
                            children.push(child);
                            if &s[o..o + 1] == "," {
                                j = o + 1;
                            } else {
                                j = o;
                            }
                        }
                    }
                }
                "<" => {
                    let mut j = i + 1;
                    let mut escape = false;
                    let mut content = Vec::new();
                    loop {
                        if escape {
                            escape = false;
                        } else {
                            match &s[j..j + 1] {
                                "!" => escape = true,
                                ">" => {
                                    return Ok((
                                        Thing::Garbage {
                                            content: content.into_iter().collect(),
                                        },
                                        j + 1,
                                    ))
                                }
                                s => content.push(s),
                            }
                        }
                        j += 1;
                    }
                }
                unknown => return Err(format!("unhandled char {}", unknown)),
            }
        }

        parse(s, 0, 1).map(|t| t.0)
    }
}

impl Thing {
    fn total_score(&self) -> usize {
        match self {
            Thing::Group { children, score } => {
                children.iter().map(|c| c.total_score()).sum::<usize>() + score
            }
            Thing::Garbage { content: _ } => 0,
        }
    }

    fn total_garbage(&self) -> usize {
        match self {
            Thing::Group { children, score: _ } => {
                children.iter().map(|c| c.total_garbage()).sum::<usize>()
            }
            Thing::Garbage { content } => content.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let examples = vec![
            ("{}", 1),
            ("{{{}}}", 6),
            ("{{},{}}", 5),
            ("{{{},{},{{}}}}", 16),
            ("{<a>,<a>,<a>,<a>}", 1),
            ("{{<ab>},{<ab>},{<ab>},{<ab>}}", 9),
            ("{{<!!>},{<!!>},{<!!>},{<!!>}}", 9),
            ("{{<a!>},{<a!>},{<a!>},{<ab>}}", 3),
        ];
        for (input, output) in examples {
            let i: Thing = input.parse().unwrap();
            assert_eq!(i.total_score(), output, "input {}", input)
        }

        let garbage = vec![
            ("<>", 0),
            ("<random characters>", 17),
            ("<<<<>", 3),
            ("<{!>}>", 2),
            ("<!!>", 0),
            ("<!!!>>", 0),
            ("<{o\"i!a,<{i<a>", 10),
        ];

        for (input, output) in garbage {
            let i: Thing = input.parse().unwrap();
            assert_eq!(i.total_garbage(), output, "input {}", input)
        }
    }
}
