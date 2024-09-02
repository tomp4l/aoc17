use std::str::FromStr;

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let instructions = lines
            .iter()
            .map(|line| line.parse())
            .collect::<Result<Vec<Instruction>, _>>()?;

        let part2 = inspect(&instructions).to_string();

        let mut coprocessor = Coprocessor::new(instructions.clone());
        coprocessor.run();

        let part1 = coprocessor.mul_count.to_string();

        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}

#[derive(Debug, Clone)]
enum ValueOrRegister {
    Value(i64),
    Register(char),
}

impl ValueOrRegister {
    fn resolve(&self, coprocessor: &Coprocessor) -> i64 {
        match self {
            ValueOrRegister::Value(value) => *value,
            ValueOrRegister::Register(register) => {
                coprocessor.registers[*register as usize - 'a' as usize]
            }
        }
    }
}

impl FromStr for ValueOrRegister {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(value) = s.parse() {
            Ok(ValueOrRegister::Value(value))
        } else {
            s.chars()
                .next()
                .map(ValueOrRegister::Register)
                .ok_or("empty string".to_string())
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Set(char, ValueOrRegister),
    Sub(char, ValueOrRegister),
    Mul(char, ValueOrRegister),
    Jnz(ValueOrRegister, ValueOrRegister),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let kind = parts.next().ok_or("empty string")?;
        let a = parts.next().ok_or("missing register")?;
        let b = parts.next().unwrap_or("");
        match kind {
            "set" => b
                .parse()
                .map(|v| Instruction::Set(a.chars().next().unwrap(), v))
                .map_err(|e| format!("failed to parse set: {}", e)),

            "mul" => b
                .parse()
                .map(|v| Instruction::Mul(a.chars().next().unwrap(), v))
                .map_err(|e| format!("failed to parse mul: {}", e)),
            "sub" => b
                .parse()
                .map(|v| Instruction::Sub(a.chars().next().unwrap(), v))
                .map_err(|e| format!("failed to parse mod: {}", e)),
            "jnz" => {
                let value = b.parse()?;
                Ok(Instruction::Jnz(a.parse()?, value))
            }
            _ => Err(format!("unknown instruction: {}", kind)),
        }
    }
}

struct Coprocessor {
    registers: [i64; 8],
    instructions: Vec<Instruction>,
    current: usize,
    mul_count: usize,
}

impl Coprocessor {
    fn new(instructions: Vec<Instruction>) -> Coprocessor {
        Coprocessor {
            registers: [0; 8],
            instructions,
            current: 0,
            mul_count: 0,
        }
    }

    fn run(&mut self) {
        while self.current < self.instructions.len() {
            match &self.instructions[self.current] {
                Instruction::Set(register, value) => {
                    self.registers[*register as usize - 'a' as usize] = value.resolve(self);
                    self.current += 1;
                }
                Instruction::Sub(register, value) => {
                    self.registers[*register as usize - 'a' as usize] -= value.resolve(self);
                    self.current += 1;
                }
                Instruction::Mul(register, value) => {
                    self.registers[*register as usize - 'a' as usize] *= value.resolve(self);
                    self.mul_count += 1;
                    self.current += 1;
                }
                Instruction::Jnz(value, offset) => {
                    if value.resolve(self) != 0 {
                        self.current = (self.current as i64 + offset.resolve(self)) as usize;
                    } else {
                        self.current += 1;
                    }
                }
            }
        }
    }
}

fn inspect(instructions: &[Instruction]) -> i64 {
    // assembly calculates composite numbers between b and c with step 17
    // assumes everyone has the same step etc and only b changes

    let b = match instructions[0] {
        Instruction::Set('b', ValueOrRegister::Value(b)) => b,
        _ => panic!("unexpected instruction"),
    };

    let mut b = 100000 + b * 100;
    let c = b + 17000;
    let mut h = 0;
    while b <= c {
        let mut f = 1;
        let mut d = 2;
        while d * d < b {
            if b % d == 0 {
                f = 0;
                break;
            }
            d += 1;
        }
        if f == 0 {
            h += 1;
        }
        b += 17;
    }
    h
}
