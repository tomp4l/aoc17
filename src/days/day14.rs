use itertools::Itertools;

use super::{day::*, knot};

pub struct Instance;

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let line = lines.first().ok_or("empty lines".to_string())?;
        let grid = make_grid(&line);

        let part1 = grid.count_used().to_string();
        let part2 = grid.count_regions().to_string();
        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}

struct Grid {
    grid: Vec<Vec<bool>>,
}

fn make_grid(seed: &str) -> Grid {
    let mut grid = Vec::new();
    for i in 0..128 {
        let hash = knot::knot_hash(&format!("{}-{}", seed, i)).dense_hash_bits();
        grid.push(hash);
    }
    Grid { grid }
}

impl Grid {
    fn count_used(&self) -> usize {
        self.grid.iter().map(|row| row.iter().counts()[&true]).sum()
    }

    fn count_regions(&self) -> usize {
        fn clear_region(grid: &mut Vec<Vec<bool>>, x: usize, y: usize) {
            if x >= 128 || y >= 128 || !grid[y][x] {
                return;
            }
            grid[y][x] = false;
            clear_region(grid, x + 1, y);
            clear_region(grid, x, y + 1);
            clear_region(grid, x.saturating_sub(1), y);
            clear_region(grid, x, y.saturating_sub(1));
        }

        let mut regions = 0;
        let mut grid = self.grid.clone();
        for y in 0..128 {
            for x in 0..128 {
                if grid[y][x] {
                    regions += 1;
                    clear_region(&mut grid, x, y);
                }
            }
        }
        regions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid() {
        let grid = make_grid("flqrgnkx");
        assert_eq!(grid.count_used(), 8108);
        assert_eq!(grid.count_regions(), 1242);
    }
}
