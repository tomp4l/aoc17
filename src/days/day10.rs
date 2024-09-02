use super::{day::*, knot};

pub struct Instance;

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let line = lines.first().ok_or("empty lines".to_string())?;
        let lengths: Vec<usize> = line
            .split(",")
            .map(|s| {
                s.parse::<usize>()
                    .map_err(|e| format!("failed to parse {}: {}", s, e))
            })
            .collect::<Result<_, _>>()?;

        let mut ring = knot::Ring::new(255);
        ring.encode(&lengths);

        let part1 = ring.first_two().to_string();

        let part2 = knot::knot_hash(&line).dense_hash();

        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}
