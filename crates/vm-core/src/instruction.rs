pub enum Instruction {
    // risc-v instruction
    Add { rd: u8, rs1: u8, rs2: u8 }, //add two registers, rd = r1+rs2
    Addi { rd: u8, rs1: u8, imm: i64 }, //addi add immediate, rd=rs1 + imm
    Sub { rd: u8, rs1: u8, rs2: u8 }, // subtract
}

impl Instruction {
    pub fn decode(inst: u32) -> Option<Self> { //this takes in a 32 bit instruction,. 
        let opcode = inst & 0x7F;//this is exactracts the lowest 7bits, which is called the opcode , tells the type of instruction it is, using bitwise and
        
        match opcode {
            0x33 => {//why specifically 0x33 cause, risc-v designers kept it as such. and every emulator uses uses this.
                // R-type: add, sub, etc.
                let rd = ((inst >> 7) & 0x1F) as u8; //we first shift it right by 7bits, to move rd to position 0, then mask with with 0x1F, using and bit wise operator and convert into a byte.
                let rs1 = ((inst >> 15) & 0x1F) as u8; //here we shift by 15bits
                let rs2 = ((inst >> 20) & 0x1F) as u8;//here by 20bits
                let funct3 = (inst >> 12) & 0x7;//here by 12 bits
                let funct7 = (inst >> 25) & 0x7F; //here by 25 bits. and why these particiular values, cause if it is a 32 bit number, first 7 are funct7, then next is rs2, then next is rs1, then fe3, then rd and the opcode 
                
                match (funct3, funct7) {
                    (0x0, 0x00) => Some(Instruction::Add { rd, rs1, rs2 }),// we check the values of funct 7 to check
                    (0x0, 0x20) => Some(Instruction::Sub { rd, rs1, rs2 }),
                    _ => None,
                }
            }
            0x13 => {
                // I-type: addi, etc.
                let rd = ((inst >> 7) & 0x1F) as u8;
                let rs1 = ((inst >> 15) & 0x1F) as u8;
                let imm = ((inst as i32) >> 20) as i64; // we first convert from u32 to i32, then shift by 20bits, then convert to i64 
                // Bit position: 31 30 29 28 27 26 25 24 23 22 21 20 | 19 18 17 16 15 | 14 13 12 | 11 10 9 8 7 | 6 5 4 3 2 1 0
//Field:        //[      imm[11:0] (12 bits)        ] [   rs1 (5)   ] [funct3(3)] [   rd (5)  ] [ opcode (7) ]

                
                Some(Instruction::Addi { rd, rs1, imm })
            }
            _ => None,
        }
    }
}
