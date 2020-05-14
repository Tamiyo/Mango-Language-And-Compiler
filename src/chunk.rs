pub type ChunkIndex = usize;
pub type InstructionIndex = usize;
pub type StackIndex = usize;
pub type ConstantIndex = usize;

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Constant(ConstantIndex),
    True,
    False,
    None,

    Add,
    Subtract,
    Negate,
    Multiply,
    Divide,
    Modulo,
    Pow,

    Not,
    NotEqual,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    JumpIfTrue(StackIndex),
    JumpIfFalse(StackIndex),
    Jump(StackIndex),

    GetGlobal(ConstantIndex),
    GetLocal(StackIndex),

    SetGlobal(ConstantIndex),
    SetLocal(StackIndex),

    Call(StackIndex),
    Function(ConstantIndex),

    Class(ConstantIndex),
    GetProperty(ConstantIndex),
    SetProperty(ConstantIndex),
    Method,

    Pop,

    List(usize),
    Slice,
    Range,
    Index,

    Print(usize),
    Return,
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub instructions: Vec<Instruction>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            instructions: vec![],
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) -> usize {
        self.instructions.push(instruction);
        self.instructions.len() - 1
    }

    pub fn patch_instruction(&mut self, index: usize) {
        let current = self.instructions.len();
        self.patch_instruction_to(index, current);
    }

    pub fn patch_instruction_to(&mut self, index: usize, to: usize) {
        match self.instructions[index] {
            Instruction::JumpIfTrue(ref mut placeholder) => *placeholder = to,
            Instruction::JumpIfFalse(ref mut placeholder) => *placeholder = to,
            Instruction::Jump(ref mut placeholder) => *placeholder = to,
            _ => panic!(format!(
                "Cannot patch instruction {:?}, ",
                self.instructions[index]
            )),
        };
    }
}
