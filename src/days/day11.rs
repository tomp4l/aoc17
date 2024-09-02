use std::str::FromStr;

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let line = lines.first().ok_or("empty lines".to_string())?;
        let directions: Vec<_> = line
            .split(",")
            .map(|l| l.parse::<Direction>())
            .collect::<Result<_, _>>()?;

        let mut grid = HexGrid::new();
        let mut max_distance = 0;
        for dir in directions {
            grid.move_dir(dir);
            if grid.distance() > max_distance {
                max_distance = grid.distance();
            }
        }

        let part1 = grid.distance().to_string();
        let part2 = Some(max_distance.to_string());
        Ok(DayResult { part1, part2 })
    }
}

struct HexGrid {
    x: i32,
    y: i32,
}

enum Direction {
    N,
    NE,
    SE,
    S,
    SW,
    NW,
}

impl HexGrid {
    fn new() -> HexGrid {
        HexGrid { x: 0, y: 0 }
    }

    fn move_dir(&mut self, dir: Direction) {
        match dir {
            Direction::N => {
                self.y += 2;
            }
            Direction::NE => {
                self.x += 1;
                self.y += 1;
            }
            Direction::SE => {
                self.x += 1;
                self.y -= 1;
            }
            Direction::S => {
                self.y -= 2;
            }
            Direction::SW => {
                self.x -= 1;
                self.y -= 1;
            }
            Direction::NW => {
                self.x -= 1;
                self.y += 1;
            }
        }
    }

    fn distance(&self) -> i32 {
        (self.x.abs() + self.y.abs()) / 2
    }
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "n" => Ok(Direction::N),
            "ne" => Ok(Direction::NE),
            "s" => Ok(Direction::S),
            "se" => Ok(Direction::SE),
            "nw" => Ok(Direction::NW),
            "sw" => Ok(Direction::SW),
            _ => Err(format!("unknown direction {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let examples = vec![
            (vec![Direction::NE, Direction::NE, Direction::NE], 3),
            (
                vec![Direction::NE, Direction::NE, Direction::SW, Direction::SW],
                0,
            ),
            (
                vec![Direction::NE, Direction::NE, Direction::S, Direction::S],
                2,
            ),
            (
                vec![
                    Direction::SE,
                    Direction::SW,
                    Direction::SE,
                    Direction::SW,
                    Direction::SW,
                ],
                3,
            ),
        ];
        for (directions, distance) in examples {
            let mut grid = HexGrid::new();
            for dir in directions {
                grid.move_dir(dir);
            }
            assert_eq!(grid.distance(), distance);
        }
    }
}
