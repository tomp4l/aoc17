use std::{collections::HashMap, mem::swap, str::FromStr};

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let mut map = lines.join("\n").parse::<Map>()?;
        let part1 = part1(&mut map).to_string();

        let mut map = lines.join("\n").parse::<Map>()?;
        let part2 = part2(&mut map).to_string();

        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coord {
    x: i64,
    y: i64,
}

enum NodeState {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl NodeState {
    fn next(&self) -> NodeState {
        match self {
            NodeState::Clean => NodeState::Infected,
            NodeState::Infected => NodeState::Clean,
            _ => panic!("Invalid state"),
        }
    }

    fn next_evolved(&self) -> NodeState {
        match self {
            NodeState::Clean => NodeState::Weakened,
            NodeState::Weakened => NodeState::Infected,
            NodeState::Infected => NodeState::Flagged,
            NodeState::Flagged => NodeState::Clean,
        }
    }
}

struct Map {
    nodes: HashMap<Coord, NodeState>,
    center: Coord,
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes = HashMap::new();
        let mut center = Coord { x: 0, y: 0 };

        let mut y = 0;
        let mut x = 0;
        for line in s.lines() {
            x = 0;
            for c in line.chars() {
                let coord = Coord { x, y };
                nodes.insert(
                    coord,
                    if c == '#' {
                        NodeState::Infected
                    } else {
                        NodeState::Clean
                    },
                );
                x += 1;
            }
            y += 1;
        }

        center.x = (x - 1) / 2;
        center.y = (y - 1) / 2;

        Ok(Map { nodes, center })
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
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
}

struct Carrier {
    coord: Coord,
    direction: Direction,
    evolved: bool,
}

impl Carrier {
    fn new(coord: Coord, evolved: bool) -> Carrier {
        Carrier {
            coord,
            direction: Direction::Up,
            evolved,
        }
    }

    fn burst(&mut self, map: &mut Map) -> bool {
        let infected = map.nodes.get(&self.coord).unwrap_or(&NodeState::Clean);

        self.direction = match infected {
            NodeState::Infected => self.direction.turn_right(),
            NodeState::Clean => self.direction.turn_left(),
            NodeState::Flagged => self.direction.turn_left().turn_left(),
            NodeState::Weakened => self.direction.clone(),
        };

        let mut new_old_coord = self.direction.move_coord(&self.coord);
        swap(&mut new_old_coord, &mut self.coord);

        let infected = if self.evolved {
            infected.next_evolved()
        } else {
            infected.next()
        };

        let infection = matches!(infected, NodeState::Infected);

        map.nodes.insert(new_old_coord, infected);

        infection
    }
}

fn part1(map: &mut Map) -> usize {
    part(map, 10_000, false)
}

fn part2(map: &mut Map) -> usize {
    part(map, 10_000_000, true)
}

fn part(map: &mut Map, iterations: usize, evolved: bool) -> usize {
    let mut carrier = Carrier::new(map.center.clone(), evolved);
    let mut infections = 0;

    for _ in 0..iterations {
        if carrier.burst(map) {
            infections += 1;
        }
    }

    infections
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "..#
#..
...";
        let mut map = input.parse::<Map>().unwrap();
        assert_eq!(part1(&mut map), 5587);
        let mut map = input.parse::<Map>().unwrap();
        assert_eq!(part(&mut map, 100, true), 26);
    }
}
