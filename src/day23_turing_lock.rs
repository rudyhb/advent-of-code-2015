use std::str::FromStr;

pub(crate) fn run() {
    let _input = _get_input();

    let instructions: Vec<Instruction> = _input.split('\n').map(|s| s.parse().unwrap()).collect();
    let mut computer = Computer::new(&instructions, 1);
    computer.execute_all();
    println!("b = {}", computer.b);
}

struct Computer<'a> {
    a: u64,
    b: u64,
    instructions: &'a [Instruction],
    current_instruction: usize,
}

impl<'a> Computer<'a> {
    pub(crate) fn new(instructions: &'a [Instruction], a: u64) -> Self {
        Self {
            a,
            b: 0,
            instructions,
            current_instruction: 0,
        }
    }
    fn get_register(&mut self, register: &Register) -> &mut u64 {
        match register {
            Register::A => &mut self.a,
            Register::B => &mut self.b
        }
    }
    fn jump(&mut self, val: isize) {
        self.current_instruction = (self.current_instruction as isize + val) as usize;
    }
    fn execute_next(&mut self) {
        let instruction = &self.instructions[self.current_instruction];
        match instruction {
            Instruction::Half(r) => {
                *self.get_register(r) /= 2;
            }
            Instruction::Triple(r) => {
                *self.get_register(r) *= 3;
            }
            Instruction::Increase(r) => {
                *self.get_register(r) += 1;
            }
            Instruction::Jump(val) => {
                self.jump(*val);
                return;
            }
            Instruction::JumpIfEven(r, val) => {
                if *self.get_register(r) % 2 == 0 {
                    self.jump(*val);
                    return;
                }
            }
            Instruction::JumpIfOne(r, val) => {
                if *self.get_register(r) == 1 {
                    self.jump(*val);
                    return;
                }
            }
        }
        self.current_instruction += 1;
    }
    pub(crate) fn execute_all(&mut self) {
        while self.current_instruction < self.instructions.len() {
            self.execute_next();
        }
    }
}

enum Register {
    A,
    B,
}

enum Instruction {
    Half(Register),
    Triple(Register),
    Increase(Register),
    Jump(isize),
    JumpIfEven(Register, isize),
    JumpIfOne(Register, isize),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_register = |s: &str| match s {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            _ => Err(())
        };

        let mut words = s.split_whitespace();
        let first = words.next().ok_or(())?;
        let second = words.next().ok_or(())?.trim_end_matches(',');
        Ok(match first {
            "hlf" => Instruction::Half(parse_register(second)?),
            "tpl" => Instruction::Triple(parse_register(second)?),
            "inc" => Instruction::Increase(parse_register(second)?),
            "jmp" => Instruction::Jump(second.parse().or(Err(()))?),
            "jie" => Instruction::JumpIfEven(parse_register(second)?, words.next().ok_or(())?.parse().or(Err(()))?),
            "jio" => Instruction::JumpIfOne(parse_register(second)?, words.next().ok_or(())?.parse().or(Err(()))?),
            _ => return Err(())
        })
    }
}

fn _get_input() -> &'static str {
    "jio a, +22
inc a
tpl a
tpl a
tpl a
inc a
tpl a
inc a
tpl a
inc a
inc a
tpl a
inc a
inc a
tpl a
inc a
inc a
tpl a
inc a
inc a
tpl a
jmp +19
tpl a
tpl a
tpl a
tpl a
inc a
inc a
tpl a
inc a
tpl a
inc a
inc a
tpl a
inc a
inc a
tpl a
inc a
tpl a
tpl a
jio a, +8
inc b
jie a, +4
tpl a
inc a
jmp +2
hlf a
jmp -7"
}