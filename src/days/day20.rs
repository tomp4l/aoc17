use std::str::FromStr;

use itertools::Itertools;

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let points = lines
            .iter()
            .map(|line| line.parse())
            .collect::<Result<Vec<Particle>, _>>()?;

        let (part1, part2) = simulate(points);
        Ok(DayResult {
            part1: part1.to_string(),
            part2: Some(part2.to_string()),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

struct Particle {
    position: Point,
    velocity: Point,
    acceleration: Point,
    collided: bool,
}

impl Particle {
    fn new(position: Point, velocity: Point, acceleration: Point) -> Particle {
        Particle {
            position,
            velocity,
            acceleration,
            collided: false,
        }
    }

    fn step(&mut self) {
        self.velocity.x += self.acceleration.x;
        self.velocity.y += self.acceleration.y;
        self.velocity.z += self.acceleration.z;
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }

    fn manhatten(&self) -> i64 {
        self.position.x.abs() + self.position.y.abs() + self.position.z.abs()
    }
}

impl FromStr for Particle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(", ");
        let position = parts
            .next()
            .ok_or("missing position".to_string())?
            .parse::<Point>()
            .map_err(|e| e.to_string())?;
        let velocity = parts
            .next()
            .ok_or("missing velocity".to_string())?
            .parse::<Point>()
            .map_err(|e| e.to_string())?;
        let acceleration = parts
            .next()
            .ok_or("missing acceleration".to_string())?
            .parse::<Point>()
            .map_err(|e| e.to_string())?;

        Ok(Particle::new(position, velocity, acceleration))
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s[3..s.len() - 1].split(",");
        let x = parts
            .next()
            .ok_or("missing x".to_string())?
            .parse::<i64>()
            .map_err(|e| e.to_string())?;
        let y = parts
            .next()
            .ok_or("missing y".to_string())?
            .parse::<i64>()
            .map_err(|e| e.to_string())?;
        let z = parts
            .next()
            .ok_or("missing z".to_string())?
            .parse::<i64>()
            .map_err(|e| e.to_string())?;

        Ok(Point { x, y, z })
    }
}

fn simulate(points: Vec<Particle>) -> (usize, usize) {
    let mut points = points;
    let mut last_closest = 0;
    let mut matched = 0;
    loop {
        for point in &mut points {
            point.step();
        }

        let closest = points
            .iter()
            .enumerate()
            .min_by_key(|(_, particle)| particle.manhatten())
            .unwrap()
            .0;

        if closest == last_closest {
            matched += 1;
        } else {
            matched = 0;
            last_closest = closest;
        }

        let mut remaining = points.iter_mut().filter(|p| !p.collided).collect_vec();
        for i in 0..remaining.len() {
            for j in i + 1..remaining.len() {
                if remaining[i].position == remaining[j].position {
                    remaining[i].collided = true;
                    remaining[j].collided = true;
                }
            }
        }

        if matched > 1000 {
            break;
        }
    }
    return (last_closest, points.iter().filter(|p| !p.collided).count());
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn example() {
        let particles: Vec<Particle> = vec![
            "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>".parse().unwrap(),
            "p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>".parse().unwrap(),
        ];

        let (part1, _) = simulate(particles);
        assert_eq!(part1, 0);

        let particles: Vec<Particle> = vec![
            "p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>".parse().unwrap(),
            "p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>".parse().unwrap(),
            "p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>".parse().unwrap(),
            "p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>".parse().unwrap(),
        ];

        let (_, part2) = simulate(particles);
        assert_eq!(part2, 1);
    }
}
