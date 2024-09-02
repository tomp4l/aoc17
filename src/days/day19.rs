use std::collections::HashMap;

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let line = parse(&lines);

        let (part1, part2) = line.traverse();
        Ok(DayResult {
            part1,
            part2: Some(part2.to_string()),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn move_coord(&self, coord: &Coord) -> Coord {
        match self {
            Direction::Up => Coord {
                x: coord.x,
                y: coord.y - 1,
            },
            Direction::Down => Coord {
                x: coord.x,
                y: coord.y + 1,
            },
            Direction::Left => Coord {
                x: coord.x - 1,
                y: coord.y,
            },
            Direction::Right => Coord {
                x: coord.x + 1,
                y: coord.y,
            },
        }
    }

    fn turn(&self, sections: &HashMap<Coord, Section>, coord: &Coord) -> Option<Direction> {
        let left = self.turn_left();
        let right = self.turn_right();
        if sections.get(&left.move_coord(&coord)).is_some() {
            Some(left)
        } else if sections.get(&right.move_coord(&coord)).is_some() {
            Some(right)
        } else {
            None
        }
    }

    fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

enum Section {
    Straight,
    Corner,
    Label(char),
}

struct Line {
    start: Coord,
    sections: HashMap<Coord, Section>,
}

impl Line {
    fn new(start: Coord) -> Line {
        Line {
            start,
            sections: HashMap::new(),
        }
    }

    fn add_section(&mut self, coord: Coord, section: Section) {
        self.sections.insert(coord, section);
    }

    fn traverse(&self) -> (String, usize) {
        let mut coord = self.start.clone();
        let mut direction = Direction::Down;
        let mut steps = 0;
        let mut letters = String::new();
        loop {
            match self.sections.get(&coord) {
                Some(Section::Straight) => {
                    coord = direction.move_coord(&coord);
                    steps += 1;
                }
                Some(Section::Corner) => {
                    if let Some(new_direction) = direction.turn(&self.sections, &coord) {
                        direction = new_direction;
                        coord = direction.move_coord(&coord);
                        steps += 1;
                    } else {
                        break;
                    }
                }
                Some(Section::Label(c)) => {
                    letters.push(*c);
                    coord = direction.move_coord(&coord);
                    steps += 1;
                }
                None => break,
            }
        }
        (letters, steps)
    }
}

fn parse(input: &[String]) -> Line {
    let mut line = Line::new(Coord { x: 0, y: 0 });
    for (y, line_str) in input.iter().enumerate() {
        for (x, c) in line_str.chars().enumerate() {
            let coord = Coord {
                x: x as i32,
                y: y as i32,
            };
            match c {
                ' ' => {}
                '|' | '-' => {
                    line.add_section(coord, Section::Straight);
                    if y == 0 {
                        line.start = Coord {
                            x: x as i32,
                            y: y as i32,
                        };
                    }
                }
                '+' => {
                    line.add_section(coord, Section::Corner);
                }
                c => {
                    line.add_section(coord, Section::Label(c));
                }
            }
        }
    }
    line
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn example() {
        let input = "     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ 
"
        .lines()
        .map(|s| s.to_string())
        .collect_vec();

        let line = parse(&input);
        assert_eq!(line.traverse(), ("ABCDEF".to_string(), 38));
    }
}
