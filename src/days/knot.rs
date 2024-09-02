use itertools::Itertools;

pub fn knot_hash(input: &str) -> Ring {
    let mut ring = Ring::new(255);
    let mut lengths = input.bytes().map(|b| b as usize).collect_vec();
    lengths.extend(&[17, 31, 73, 47, 23]);

    for _ in 0..64 {
        ring.encode(&lengths);
    }

    ring
}

pub struct Ring {
    list: Vec<u8>,
    position: usize,
    skip_size: usize,
}

impl Ring {
    pub fn new(max_num: u8) -> Ring {
        Ring {
            list: (0..=max_num).collect_vec(),
            position: 0,
            skip_size: 0,
        }
    }

    pub fn encode(&mut self, lengths: &[usize]) {
        let skip = &mut self.skip_size;
        let i = &mut self.position;
        let len = self.list.len();

        for l in lengths {
            let end = *i + l - 1;
            for j in 0..(l / 2) {
                let from = *i + j;
                let to = end - j;
                self.list.swap(from % len, to % len);
            }

            *i += l + *skip;
            *i %= self.list.len();
            *skip += 1;
        }
    }

    pub fn first_two(&self) -> u16 {
        self.list[0] as u16 * self.list[1] as u16
    }

    pub fn dense_hash(&self) -> String {
        self.list
            .chunks(16)
            .map(|w| w.iter().fold(0, |a, b| a ^ b))
            .map(|i| format!("{:0>2x}", i))
            .collect()
    }

    pub fn dense_hash_bits(&self) -> Vec<bool> {
        self.list
            .chunks(16)
            .map(|w| w.iter().fold(0, |a, b| a ^ b))
            .flat_map(|i| (0..8).map(move |j| (i >> (7 - j)) & 1 == 1))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let input = vec![3, 4, 1, 5];
        let mut ring = Ring::new(4);
        ring.encode(&input);
        assert_eq!(ring.first_two(), 12)
    }

    #[test]
    fn knot_hash_examples() {
        let examples = vec![
            ("", "a2582a3a0e66e6e86e3812dcb672a272", 60),
            ("AoC 2017", "33efeb34ea91902bb2f59c9920caa6cd", 66),
            ("1,2,3", "3efbe78a8d82f29979031a4aa0b16a9d", 64),
            ("1,2,4", "63960835bcdc130f0b66d7ff4f6a5a8e", 68),
        ];
        for (i, o, b) in examples {
            assert_eq!(knot_hash(i).dense_hash(), o, "input: {}", i);
            assert_eq!(knot_hash(i).dense_hash_bits().len(), 128);
            assert_eq!(
                knot_hash(i)
                    .dense_hash_bits()
                    .iter()
                    .filter(|&&b| b)
                    .count(),
                b
            );
        }
    }
}
