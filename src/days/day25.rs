use std::{collections::HashMap, str::FromStr, usize};

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let mut machine: TuringMachine = lines.join("\n").parse()?;

        let part1 = machine.checksum().to_string();
        Ok(DayResult { part1, part2: None })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum State {
    A,
    B,
    C,
    D,
    E,
    F,
}

struct Transition {
    write: bool,
    move_right: bool,
    next_state: State,
}

struct TuringMachine {
    tape_right: Vec<bool>,
    tape_left: Vec<bool>,
    cursor: i64,
    state: State,
    transitions: HashMap<(State, bool), Transition>,
    checksum_after: usize,
}

impl FromStr for TuringMachine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let initial_state = lines
            .next()
            .ok_or("missing initial state")?
            .split_whitespace()
            .nth(3)
            .ok_or("missing initial state value")?
            .chars()
            .next()
            .ok_or("missing initial state value")?;
        let initial_state = match initial_state {
            'A' => State::A,
            'B' => State::B,
            'C' => State::C,
            'D' => State::D,
            'E' => State::E,
            'F' => State::F,
            _ => return Err("invalid initial state value".to_string()),
        };

        let checksum_after = lines
            .next()
            .ok_or("missing checksum after")?
            .split_whitespace()
            .nth(5)
            .ok_or("missing checksum after value")?
            .parse::<usize>()
            .map_err(|e| format!("invalid checksum after value: {}", e))?;

        let mut transitions = HashMap::new();
        while let Some(_) = lines.next() {
            let state = match lines
                .next()
                .ok_or("missing state")?
                .split_whitespace()
                .nth(2)
                .ok_or("missing state value")?
                .trim_end_matches(":")
            {
                "A" => State::A,
                "B" => State::B,
                "C" => State::C,
                "D" => State::D,
                "E" => State::E,
                "F" => State::F,
                s => return Err(format!("invalid state value: {}", s)),
            };

            let (t, v) = parse_transition(&mut lines)?;
            transitions.insert((state, v), t);
            let (t, v) = parse_transition(&mut lines)?;
            transitions.insert((state, v), t);
        }

        Ok(TuringMachine {
            tape_right: vec![false],
            tape_left: vec![],
            cursor: 0,
            state: initial_state,
            transitions,
            checksum_after,
        })
    }
}

fn parse_transition<'a, I: Iterator<Item = &'a str>>(
    lines: &mut I,
) -> Result<(Transition, bool), String> {
    let value = match lines
        .next()
        .ok_or("missing value")?
        .split_whitespace()
        .nth(5)
        .ok_or("missing value value")?
        .trim_end_matches(":")
    {
        "0" => false,
        "1" => true,
        v => return Err(format!("invalid value value {}", v)),
    };

    let write = match lines
        .next()
        .ok_or("missing value")?
        .split_whitespace()
        .nth(4)
        .ok_or("missing value value")?
        .trim_end_matches(".")
    {
        "0" => false,
        "1" => true,
        v => return Err(format!("invalid write value {}", v)),
    };

    let move_right = match lines
        .next()
        .ok_or("missing value")?
        .split_whitespace()
        .nth(6)
        .ok_or("missing move value")?
        .trim_end_matches(".")
    {
        "right" => true,
        "left" => false,
        v => return Err(format!("invalid move value {}", v)),
    };

    let next_state = match lines
        .next()
        .ok_or("missing value")?
        .split_whitespace()
        .nth(4)
        .ok_or("missing value value")?
        .trim_end_matches(".")
    {
        "A" => State::A,
        "B" => State::B,
        "C" => State::C,
        "D" => State::D,
        "E" => State::E,
        "F" => State::F,
        v => return Err(format!("invalid state transition value {}", v)),
    };

    Ok((
        Transition {
            write,
            move_right,
            next_state,
        },
        value,
    ))
}

impl TuringMachine {
    fn checksum(&mut self) -> usize {
        for _ in 0..self.checksum_after {
            let current = self.get_current();
            let &Transition {
                write,
                move_right,
                next_state,
            } = &self.transitions[&(self.state, current)];
            self.state = next_state;
            self.set_current(write);
            if move_right {
                self.cursor += 1;
            } else {
                self.cursor -= 1;
            }
        }

        self.tape_left.iter().filter(|t| **t).count()
            + self.tape_right.iter().filter(|t| **t).count()
    }

    fn get_current(&mut self) -> bool {
        if self.cursor >= 0 {
            let i = self.cursor as usize;
            if i < self.tape_right.len() {
                self.tape_right[i]
            } else {
                self.tape_right.push(false);
                false
            }
        } else {
            let i = (-1 - self.cursor) as usize;
            if i < self.tape_left.len() {
                self.tape_left[i]
            } else {
                self.tape_left.push(false);
                false
            }
        }
    }

    fn set_current(&mut self, write: bool) {
        if self.cursor >= 0 {
            self.tape_right[self.cursor as usize] = write
        } else {
            self.tape_left[(-1 - self.cursor) as usize] = write
        }
    }
}
