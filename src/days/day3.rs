use std::collections::HashMap;

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let parsed = lines
            .first()
            .unwrap_or(&"".to_string())
            .parse::<u32>()
            .map_err(|e| e.to_string())?;

        let part1 = steps(parsed).to_string();
        let part2 = allocate(parsed).to_string();

        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}

struct Spiral {
    pos: (i32, i32),
    dir: u8,
    size: i32,
}

impl Iterator for Spiral {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.pos;
        let change_dir = match self.dir {
            0 => {
                self.pos.1 -= 1;
                self.pos.1 == -self.size
            }
            1 => {
                self.pos.0 -= 1;
                self.pos.0 == -self.size
            }
            2 => {
                self.pos.1 += 1;
                self.pos.1 == self.size
            }
            3 => {
                self.pos.0 += 1;
                self.pos.0 == self.size + 1
            }
            _ => unreachable!(),
        };
        if change_dir {
            self.dir += 1;
        }
        if self.dir == 4 {
            self.dir = 0;
            self.size += 1;
        }

        Some(pos)
    }
}

impl Spiral {
    fn new() -> Self {
        Spiral {
            pos: (0, 0),
            dir: 3,
            size: 0,
        }
    }
}

fn steps(number: u32) -> u32 {
    let pos = Spiral::new()
        .nth(number as usize - 1)
        .expect("infinite iterator");
    pos.0.abs() as u32 + pos.1.abs() as u32
}

fn allocate(target: u32) -> u32 {
    let mut allocated: HashMap<(i32, i32), u32> = HashMap::new();
    allocated.insert((0, 0), 1);

    for pos in Spiral::new().skip(1) {
        let mut value = 0;

        for i in -1..=1 {
            for j in -1..=1 {
                if i != 0 || j != 0 {
                    value += allocated
                        .get(&(pos.0 + i, pos.1 + j))
                        .cloned()
                        .unwrap_or_default();
                }
            }
        }

        if value > target {
            return value;
        }

        allocated.insert(pos, value);
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gives_correct_steps() {
        assert_eq!(steps(1), 0);
        assert_eq!(steps(12), 3);
        assert_eq!(steps(23), 2);
        assert_eq!(steps(1024), 31);
    }

    #[test]
    fn it_gives_correct_allocation() {
        assert_eq!(allocate(1), 2);
        assert_eq!(allocate(804), 806);
    }
}
