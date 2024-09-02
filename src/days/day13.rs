use std::collections::HashMap;

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let firewall = parse(&lines)?;

        let part1 = firewall.severity().to_string();
        let part2 = firewall.delay().to_string();
        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}

fn parse(input: &[String]) -> Result<Firewall, String> {
    let mut firewall = Firewall::new();
    for line in input {
        let parts: Vec<_> = line.split(": ").collect();
        let depth = parts[0]
            .parse::<usize>()
            .map_err(|e| format!("bad depth {}: {}", line, e))?;
        let range = parts[1]
            .parse::<usize>()
            .map_err(|e| format!("bad range {}: {}", line, e))?;
        firewall.add_layer(depth, range);
    }
    Ok(firewall)
}

struct Firewall {
    depth_range: HashMap<usize, usize>,
}

impl Firewall {
    fn new() -> Self {
        Firewall {
            depth_range: HashMap::new(),
        }
    }

    fn add_layer(&mut self, depth: usize, range: usize) {
        self.depth_range.insert(depth, range);
    }

    fn severity(&self) -> usize {
        let mut severity = 0;
        for (depth, range) in &self.depth_range {
            if depth % (2 * (range - 1)) == 0 {
                severity += depth * range;
            }
        }
        severity
    }

    fn delay(&self) -> usize {
        let mut delay = 0;
        loop {
            if self
                .depth_range
                .iter()
                .all(|(depth, range)| (depth + delay) % (2 * (range - 1)) != 0)
            {
                return delay;
            }
            delay += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input() {
        let input = vec![
            "0: 3".to_owned(),
            "1: 2".to_owned(),
            "4: 4".to_owned(),
            "6: 4".to_owned(),
        ];
        let firewall = parse(&input).unwrap();
        assert_eq!(firewall.severity(), 24);
        assert_eq!(firewall.delay(), 10);
    }
}
