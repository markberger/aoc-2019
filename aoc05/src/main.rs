use std::env;
use std::error;
use std::fs;

enum ParameterMode {
    Position,
    Immediate,
}

impl From<i32> for ParameterMode {
    fn from(i: i32) -> Self {
        match i {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => unreachable!(),
        }
    }
}

struct Parameter {
    mode: ParameterMode,
    val: i32,
}

impl Parameter {
    fn load(&self, memory: &Vec<i32>) -> i32 {
        match self.mode {
            ParameterMode::Position => memory[self.val as usize],
            ParameterMode::Immediate => self.val,
        }
    }
}

struct Output(usize);

impl Output {
    fn write(&self, memory: &mut Vec<i32>, value: i32) {
        memory[self.0] = value;
    }
}

enum Instruction {
    Add(Parameter, Parameter, Output),
    Multiply(Parameter, Parameter, Output),
    Input(Output),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Output),
    Equal(Parameter, Parameter, Output),
    Terminate,
}

impl Instruction {
    fn parse(program: &mut Program) -> Instruction {
        let coded_instr = program.memory[program.ptr];
        let opcode = coded_instr % 100;
        let mode_a = ParameterMode::from((coded_instr / 100) % 10);
        let mode_b = ParameterMode::from((coded_instr / 1000) % 10);

        match opcode {
            1 => Instruction::Add(
                Parameter {
                    mode: mode_a,
                    val: program.memory[program.ptr + 1],
                },
                Parameter {
                    mode: mode_b,
                    val: program.memory[program.ptr + 2],
                },
                Output {
                    0: program.memory[program.ptr + 3] as usize,
                },
            ),
            2 => Instruction::Multiply(
                Parameter {
                    mode: mode_a,
                    val: program.memory[program.ptr + 1],
                },
                Parameter {
                    mode: mode_b,
                    val: program.memory[program.ptr + 2],
                },
                Output {
                    0: program.memory[program.ptr + 3] as usize,
                },
            ),
            3 => Instruction::Input(Output {
                0: program.memory[program.ptr + 1] as usize,
            }),
            4 => Instruction::Output(Parameter {
                mode: mode_a,
                val: program.memory[program.ptr + 1],
            }),
            5 => Instruction::JumpIfTrue(
                Parameter {
                    mode: mode_a,
                    val: program.memory[program.ptr + 1],
                },
                Parameter {
                    mode: mode_b,
                    val: program.memory[program.ptr + 2],
                },
            ),
            6 => Instruction::JumpIfFalse(
                Parameter {
                    mode: mode_a,
                    val: program.memory[program.ptr + 1],
                },
                Parameter {
                    mode: mode_b,
                    val: program.memory[program.ptr + 2],
                },
            ),
            7 => Instruction::LessThan(
                Parameter {
                    mode: mode_a,
                    val: program.memory[program.ptr + 1],
                },
                Parameter {
                    mode: mode_b,
                    val: program.memory[program.ptr + 2],
                },
                Output {
                    0: program.memory[program.ptr + 3] as usize,
                },
            ),
            8 => Instruction::Equal(
                Parameter {
                    mode: mode_a,
                    val: program.memory[program.ptr + 1],
                },
                Parameter {
                    mode: mode_b,
                    val: program.memory[program.ptr + 2],
                },
                Output {
                    0: program.memory[program.ptr + 3] as usize,
                },
            ),
            99 => Instruction::Terminate,
            _ => unreachable!(),
        }
    }

    fn size(&self) -> usize {
        match self {
            Self::Add { .. }
            | Self::Multiply { .. }
            | Self::LessThan { .. }
            | Self::Equal { .. } => 4,
            Self::Input { .. } | Self::Output { .. } => 2,
            Self::JumpIfTrue { .. } | Self::JumpIfFalse { .. } => 3,
            Self::Terminate => 1,
        }
    }

    fn is_terminate(&self) -> bool {
        match self {
            Self::Terminate => true,
            _ => false,
        }
    }
}

#[derive(PartialEq)]
enum ProgramState {
    Running,
    Halted,
}

struct Program {
    ptr: usize,
    memory: Vec<i32>,
    inputs: Vec<i32>,
}

impl Program {
    fn new(memory: Vec<i32>, inputs: Vec<i32>) -> Program {
        let mut p = Program {
            ptr: 0,
            memory: memory,
            inputs: inputs,
        };
        p.inputs.reverse();
        return p;
    }

    fn execute(&mut self, instr: &Instruction) {
        match instr {
            Instruction::Add(p1, p2, output) => {
                let v1 = p1.load(&self.memory);
                let v2 = p2.load(&self.memory);
                output.write(&mut self.memory, v1 + v2);
            }
            Instruction::Multiply(p1, p2, output) => {
                let v1 = p1.load(&self.memory);
                let v2 = p2.load(&self.memory);
                output.write(&mut self.memory, v1 * v2);
            }
            Instruction::Input(output) => {
                let v = self.inputs.pop().expect("missing input value");
                output.write(&mut self.memory, v);
            }
            Instruction::Output(p) => {
                let v = p.load(&self.memory);
                println!("{}", v);
            }
            Instruction::JumpIfTrue(p1, p2) => {
                let v_cond = p1.load(&self.memory);
                let v_ptr = p2.load(&self.memory);
                if v_cond != 0 {
                    self.ptr = v_ptr as usize;
                }
            }
            Instruction::JumpIfFalse(p1, p2) => {
                let v_cond = p1.load(&self.memory);
                let v_ptr = p2.load(&self.memory);
                if v_cond == 0 {
                    self.ptr = v_ptr as usize;
                }
            }
            Instruction::LessThan(p1, p2, output) => {
                let v1 = p1.load(&self.memory);
                let v2 = p2.load(&self.memory);
                if v1 < v2 {
                    output.write(&mut self.memory, 1);
                } else {
                    output.write(&mut self.memory, 0);
                }
            }
            Instruction::Equal(p1, p2, output) => {
                let v1 = p1.load(&self.memory);
                let v2 = p2.load(&self.memory);
                if v1 == v2 {
                    output.write(&mut self.memory, 1);
                } else {
                    output.write(&mut self.memory, 0);
                }
            }
            Instruction::Terminate => {}
        }
    }

    fn tick(&mut self) -> ProgramState {
        let instr = Instruction::parse(self);
        self.ptr += instr.size();
        self.execute(&instr);

        if instr.is_terminate() {
            ProgramState::Halted
        } else {
            ProgramState::Running
        }
    }

    fn run(&mut self) {
        loop {
            let state = self.tick();
            if state == ProgramState::Halted {
                return;
            }
        }
    }
}

fn parse_input(filename: String) -> Result<Vec<i32>, Box<dyn error::Error>> {
    let input = fs::read_to_string(filename)?
        .trim()
        .split(',')
        .map(|elem| elem.parse())
        .collect::<Result<Vec<i32>, _>>()?;

    Ok(input)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let memory = parse_input(args[1].clone()).unwrap();
    let inputs = vec![5];
    let mut program = Program::new(memory, inputs);
    program.run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter() {
        let mut memory: Vec<i32> = vec![8];
        let mut p1 = Parameter {
            mode: ParameterMode::Immediate,
            val: 0,
        };
        assert_eq!(p1.load(&mut memory), 0);

        p1.mode = ParameterMode::Position;
        assert_eq!(p1.load(&mut memory), 8);
        println!("test");
    }
}
