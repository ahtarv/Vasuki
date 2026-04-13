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
            self.execute(instruction, memory);
        }
        self.pc+=4;
    }
    //we added memory later after we added lb ld sb cause add and sub dont need memory they work on registers where load and store need memory to save to RAM
    fn execute(&mut self, instruction: Instruction, memory: &mut Memory) {
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

#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn test_addi(){
        let mut cpu = Cpu::new();
        let mut mem =  Memory::new(1024);

        let instruction: u32 = (42 << 20) | (0 << 15) | (0 << 12) | (5 << 7) | 0x13;
        mem.write_u32(0, instruction);

        cpu.step(&mut mem);

        assert_eq!(cpu.regs[5], 42);
        assert_eq!(cpu.pc, 4);
    }

    #[test]
    fn test_add(){
        let mut cpu = Cpu::new();
        let mut mem = Memory::new(1024);

        cpu.regs[1] = 10;
        cpu.regs[2] = 20;

        let instruction: u32 = (0<<25) | (2<<20) | (1<<15) | (0<<12) | (3<<7) | 0x33;
        mem.write_u32(0, instruction);

        cpu.step(&mut mem);
        assert_eq!(cpu.regs[3], 30);
    }
}