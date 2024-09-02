use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, input: Vec<String>) -> Result<DayResult, String> {
        let parsed = input
            .iter()
            .map(|i| i.parse::<Instruction>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let mut computer = Computer::new();
        computer.run(&parsed);

        let part1 = computer.max_reg().to_string();
        Ok(DayResult {
            part1,
            part2: Some(computer.max_ever.to_string()),
        })
    }
}

struct Computer {
    registers: HashMap<String, i32>,
    max_ever: i32,
}

#[derive(PartialEq, Eq, Debug)]
enum Cond {
    Gt,
    Lt,
    Gte,
    Lte,
    Eq,
    Neq,
}

#[derive(PartialEq, Eq, Debug)]
struct Instruction {
    reg: String,
    inc: bool,
    amount: i32,
    cond_reg: String,
    cond_type: Cond,
    cond_amount: i32,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect_vec();
        if parts.len() != 7 {
            return Err("".to_owned());
        }

        let reg = parts[0].to_owned();
        let inc = parts[1] == "inc";
        let amount = parts[2]
            .parse()
            .map_err(|e| format!("failed to parse amount {}: {}", parts[2], e))?;

        let cond_reg = parts[4].to_owned();
        let cond_type = match parts[5] {
            ">" => Cond::Gt,
            "<" => Cond::Lt,
            ">=" => Cond::Gte,
            "<=" => Cond::Lte,
            "==" => Cond::Eq,
            "!=" => Cond::Neq,
            unknown => return Err(format!("Unknown condition {}", unknown)),
        };
        let cond_amount = parts[6]
            .parse()
            .map_err(|e| format!("failed to parse amount {}: {}", parts[2], e))?;

        Ok(Instruction {
            reg,
            inc,
            amount,
            cond_reg,
            cond_type,
            cond_amount,
        })
    }
}

impl Computer {
    fn new() -> Self {
        Computer {
            registers: HashMap::new(),
            max_ever: i32::MIN,
        }
    }

    fn run(&mut self, instructions: &[Instruction]) {
        for i in instructions {
            let reg_value = self.get_reg(&i.reg);
            let cond_reg_value = self.get_reg(&i.cond_reg);

            let passes = match i.cond_type {
                Cond::Gt => cond_reg_value > i.cond_amount,
                Cond::Lt => cond_reg_value < i.cond_amount,
                Cond::Gte => cond_reg_value >= i.cond_amount,
                Cond::Lte => cond_reg_value <= i.cond_amount,
                Cond::Eq => cond_reg_value == i.cond_amount,
                Cond::Neq => cond_reg_value != i.cond_amount,
            };

            if passes {
                let new_amount = if i.inc {
                    reg_value + i.amount
                } else {
                    reg_value - i.amount
                };

                if reg_value > self.max_ever {
                    self.max_ever = reg_value;
                }

                self.registers.insert(i.reg.clone(), new_amount);
            }
        }
    }

    fn get_reg(&self, reg: &str) -> i32 {
        self.registers.get(reg).copied().unwrap_or_default()
    }

    fn max_reg(&self) -> i32 {
        self.registers.values().max().copied().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_instructions() {
        assert_eq!(
            "b inc 5 if a > 1".parse::<Instruction>(),
            Ok(Instruction {
                reg: "b".to_owned(),
                inc: true,
                amount: 5,
                cond_reg: "a".to_owned(),
                cond_type: Cond::Gt,
                cond_amount: 1
            })
        )
    }

    #[test]
    fn example() {
        let input = vec![
            "b inc 5 if a > 1",
            "a inc 1 if b < 5",
            "c dec -10 if a >= 1",
            "c inc -20 if c == 10",
        ];
        let parsed = input
            .iter()
            .map(|i| i.parse::<Instruction>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let mut computer = Computer::new();
        computer.run(&parsed);
        assert_eq!(computer.get_reg("a"), 1);
        assert_eq!(computer.get_reg("b"), 0);
        assert_eq!(computer.get_reg("c"), -10);
        assert_eq!(computer.max_reg(), 1);
        assert_eq!(computer.max_ever, 10);
    }
}
