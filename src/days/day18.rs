use std::{collections::VecDeque, str::FromStr};

use super::day::*;

pub struct Instance;

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let instructions = lines
            .iter()
            .map(|line| line.parse())
            .collect::<Result<Vec<Instruction>, _>>()?;

        let mut duet = Duet::new(instructions.clone());

        let output = duet.run(true);
        let part1 = output
            .sends
            .iter()
            .last()
            .copied()
            .unwrap_or_default()
            .to_string();
        let part2 = run_duet(instructions).to_string();

        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}

type Value = i64;
type Register = char;

#[derive(Debug, Clone)]
enum ValueOrRegister {
    Value(Value),
    Register(Register),
}

struct Registers {
    registers: [Value; 26],
}

impl Registers {
    fn get(&self, register: &Register) -> Value {
        self.registers[*register as usize - 'a' as usize]
    }

    fn set(&mut self, register: &Register, value: Value) {
        self.registers[*register as usize - 'a' as usize] = value;
    }

    fn add(&mut self, register: &Register, value: Value) {
        self.registers[*register as usize - 'a' as usize] += value;
    }

    fn mul(&mut self, register: &Register, value: Value) {
        self.registers[*register as usize - 'a' as usize] *= value;
    }

    fn modulo(&mut self, register: &Register, value: Value) {
        self.registers[*register as usize - 'a' as usize] %= value;
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

impl ValueOrRegister {
    fn resolve(&self, registers: &Registers) -> Value {
        match self {
            ValueOrRegister::Value(v) => *v,
            ValueOrRegister::Register(r) => registers.get(r),
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Snd(ValueOrRegister),
    Set(Register, ValueOrRegister),
    Add(Register, ValueOrRegister),
    Mul(Register, ValueOrRegister),
    Mod(Register, ValueOrRegister),
    Rcv(Register),
    Jgz(ValueOrRegister, ValueOrRegister),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let kind = parts.next().ok_or("empty string")?;
        let a = parts.next().ok_or("missing register")?;
        let b = parts.next().unwrap_or("");
        match kind {
            "snd" => a
                .parse()
                .map(Instruction::Snd)
                .map_err(|e| format!("failed to parse snd: {}", e)),
            "set" => b
                .parse()
                .map(|v| Instruction::Set(a.chars().next().unwrap(), v))
                .map_err(|e| format!("failed to parse set: {}", e)),
            "add" => b
                .parse()
                .map(|v| Instruction::Add(a.chars().next().unwrap(), v))
                .map_err(|e| format!("failed to parse add: {}", e)),
            "mul" => b
                .parse()
                .map(|v| Instruction::Mul(a.chars().next().unwrap(), v))
                .map_err(|e| format!("failed to parse mul: {}", e)),
            "mod" => b
                .parse()
                .map(|v| Instruction::Mod(a.chars().next().unwrap(), v))
                .map_err(|e| format!("failed to parse mod: {}", e)),
            "rcv" => Ok(Instruction::Rcv(a.chars().next().unwrap())),
            "jgz" => {
                let value = b.parse()?;
                Ok(Instruction::Jgz(a.parse()?, value))
            }
            _ => Err(format!("unknown instruction: {}", kind)),
        }
    }
}

struct Duet {
    registers: Registers,
    instructions: Vec<Instruction>,
    pc: usize,
    value_queue: VecDeque<Value>,
    values_sent: usize,
}

struct Output {
    terminated: bool,
    sends: Vec<Value>,
}

impl Duet {
    fn new(instructions: Vec<Instruction>) -> Duet {
        Duet {
            registers: Registers { registers: [0; 26] },
            instructions,
            pc: 0,
            value_queue: Default::default(),
            values_sent: 0,
        }
    }

    fn run(&mut self, sound: bool) -> Output {
        let mut sounds = vec![];
        while let Some(instruction) = self.instructions.get(self.pc) {
            match instruction {
                Instruction::Snd(value) => {
                    let value = value.resolve(&self.registers);
                    sounds.push(value);
                    self.values_sent += 1;
                }
                Instruction::Set(register, value) => {
                    let value = value.resolve(&self.registers);
                    self.registers.set(register, value);
                }
                Instruction::Add(register, value) => {
                    let value = value.resolve(&self.registers);
                    self.registers.add(register, value);
                }
                Instruction::Mul(register, value) => {
                    let value = value.resolve(&self.registers);
                    self.registers.mul(register, value);
                }
                Instruction::Mod(register, value) => {
                    let value = value.resolve(&self.registers);
                    self.registers.modulo(register, value);
                }
                Instruction::Rcv(register) => {
                    if sound {
                        let value = self.registers.get(register);
                        if value > 0 {
                            break;
                        }
                    } else {
                        if let Some(value) = self.value_queue.pop_front() {
                            self.registers.set(register, value);
                        } else {
                            break;
                        }
                    }
                }
                Instruction::Jgz(value, offset) => {
                    let value = value.resolve(&self.registers);
                    let offset = offset.resolve(&self.registers);
                    if value > 0 {
                        self.pc = (self.pc as i64 + offset) as usize;
                        continue;
                    }
                }
            }
            self.pc += 1;
        }
        Output {
            terminated: self.pc >= self.instructions.len(),
            sends: sounds,
        }
    }
}

fn run_duet(instructions: Vec<Instruction>) -> usize {
    let mut duet0 = Duet::new(instructions.clone());
    let mut duet1 = Duet::new(instructions);

    duet0.registers.set(&'p', 0);
    duet1.registers.set(&'p', 1);

    loop {
        let output0 = duet0.run(false);
        let output1 = duet1.run(false);

        duet0.value_queue.extend(output1.sends.iter().copied());
        duet1.value_queue.extend(output0.sends.iter().copied());

        if duet0.value_queue.is_empty() && duet1.value_queue.is_empty() {
            break;
        }
        if output0.terminated && output1.terminated {
            break;
        }
    }

    duet1.values_sent
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn example() {
        let instuctions = vec![
            "set a 1".parse().unwrap(),
            "add a 2".parse().unwrap(),
            "mul a a".parse().unwrap(),
            "mod a 5".parse().unwrap(),
            "snd a".parse().unwrap(),
            "set a 0".parse().unwrap(),
            "rcv a".parse().unwrap(),
            "jgz a -1".parse().unwrap(),
            "set a 1".parse().unwrap(),
            "jgz a -2".parse().unwrap(),
        ];

        let mut duet = Duet::new(instuctions);
        let output = duet.run(true);

        assert_eq!(output.sends, vec![4]);
    }

    #[test]
    fn duet_example() {
        let instructions = vec![
            "snd 1".parse().unwrap(),
            "snd 2".parse().unwrap(),
            "snd p".parse().unwrap(),
            "rcv a".parse().unwrap(),
            "rcv b".parse().unwrap(),
            "rcv c".parse().unwrap(),
            "rcv d".parse().unwrap(),
        ];

        let result = run_duet(instructions);

        assert_eq!(result, 3);
    }
}
