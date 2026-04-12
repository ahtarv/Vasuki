use crate::{Memory, Instruction};

pub struct Cpu{ //how you declare structres in rust
    pub regs: [u64; 32], //array of 32 elements with unsigned 64 bit integer being the data type
    pub pc: u64, //a public field for the counter, also a 64 bit unsigned integer
}

impl Cpu{ //called the implementation block
    pub fn new()-> Self{ //here is the constructor, something that called immediately when the impl is called
        Self{
            regs: [0;32], //initilizes a 32 bit array with allow of them as zeros
            pc:0,// program counter set to zero
            //note the last expression in rust is automatically retuurned, no need for return keyword
        }
    }
    pub fn reset(&mut self)
    {
        self.regs = [0;32];//resets all registers to zero
        self.pc=0;//reset the program counter to zero
    }

    pub fn fetch(&self, memory: &Memory) -> u32 {
        memory.read_u32(self.pc)
    }

    pub fn step(&mut self, memory: &mut Memory){
        let inst_bits = self.fetch(memory);
        if let Some(instruction) = Instruction::decode(inst_bits) {
            self.execute(instruction);
        }
        self.pc+=4;
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction{
            Instruction::Add{rd, rs1, rs2} => {
                if rd!=0 {
                    self.regs[rd as usize] = self.regs[rs1 as usize]
                            .wrapping_add(self.regs[rs2 as usize]);
                }
            }
            Instruction::Addi{rd, rs1, imm} => {
                if rd!=0 {
                    self.regs[rd as usize] = self.regs[rs1 as usize]
                        .wrapping_add(imm as u64);
                }
            }
            Instruction::Sub{rd, rs1, rs2} => {
                if rd!=0 {
                    self.regs[rd as usize] = self.regs[rs1 as usize]
                        .wrapping_sub(self.regs[rs2 as usize]);
                }
            }
        }
    }
}