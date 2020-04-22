use crate::chunk::{Chunk, Instruction};
use crate::constant::Constant;

impl Chunk {
    pub fn disassemble(&self, constants: &[Constant]) {
        println!("{:<8}\t\t{:<16}\t{:<16}", "OFFSET", "INSTRUCTION", "OPERAND");
        for (index, instruction) in self.instructions.clone().into_iter().enumerate() {
            match instruction {
                Instruction::Constant(offset) => constant(index, Instruction::Constant(offset), constants[offset].clone()),
                Instruction::True => simple(index, Instruction::True),
                Instruction::False => simple(index, Instruction::False),
                Instruction::None => simple(index, Instruction::None),

                Instruction::Add => simple(index, Instruction::Add),
                Instruction::Subtract => simple(index, Instruction::Subtract),
                Instruction::Multiply => simple(index, Instruction::Multiply),
                Instruction::Divide => simple(index, Instruction::Divide),
                Instruction::Modulo => simple(index, Instruction::Modulo),
                Instruction::Pow => simple(index, Instruction::Pow),
                Instruction::Not => simple(index, Instruction::Not),
                Instruction::NotEqual => simple(index, Instruction::NotEqual),
                Instruction::EqualEqual => simple(index, Instruction::EqualEqual),
                Instruction::Greater => simple(index, Instruction::Greater),
                Instruction::GreaterEqual => simple(index, Instruction::GreaterEqual),
                Instruction::Less => simple(index, Instruction::Less),
                Instruction::LessEqual => simple(index, Instruction::LessEqual),

                Instruction::Pop => simple(index, Instruction::Pop),
                Instruction::Print => simple(index, Instruction::Print),
                Instruction::Return => simple(index, Instruction::Return),

                Instruction::GetGlobal(offset) => simple(index, Instruction::GetGlobal(offset)),
                Instruction::SetGlobal(offset) => simple(index, Instruction::SetGlobal(offset)),

                Instruction::GetLocal(offset) => simple(index, Instruction::GetLocal(offset)),
                Instruction::SetLocal(offset) => simple(index, Instruction::SetLocal(offset)),

                Instruction::JumpIfTrue(offset) => simple(index, Instruction::JumpIfTrue(offset)),
                Instruction::JumpIfFalse(offset) => simple(index, Instruction::JumpIfFalse(offset)),
                Instruction::Jump(offset) => simple(index, Instruction::Jump(offset)),

                Instruction::Call(args) => simple(index, Instruction::Call(args)),
                Instruction::Function(offset) => simple(index, Instruction::Function(offset)),

                _ => panic!(format!("Unknown instruction {:?}", instruction))
            }
        }
    }
}

fn simple(offset: usize, instruction: Instruction) {
    println!("{:<#008x}\t\t{:<16}\t{:<16}", offset, format!("{:?}", instruction), "");
}

fn constant(offset: usize, instruction: Instruction, constant: Constant) {
    println!("{:#008x}\t\t{:<16}\t{:<16}", offset, format!("{:?}", instruction), format!("{:?}", constant));
}