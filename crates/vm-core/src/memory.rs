pub struct Memory{
    ram: Vec<u8>, //creates a vector with each elemnt being an 8bit integer
}

impl Memory{
    pub fn new(size: usize) -> Self{//size:usize, is the parameter for how many bytes of RAM, 
        Self{
            ram:vec![0;size], //creates a vector with size amt of zeros, its a constructor
        }
    }

    pub fn read_u8(&self, addr: u64) -> u8{ //and self is the immutable borrow, we are just reading not borrowing, addr: u64 is the memory address as 64 bit number. and ->u8 is the return type
        self.ram[addr as usize] // here we are type casting as usize, 
    }
    pub fn write_u8(&mut self, addr: u64, value: u8){ //value: u8, this is a parameter.
        self.ram[addr as usize] = value;
    }
    pub fn read_u32(&self, addr: u64) -> u32{
        let addr = addr as usize; //convert once and reuse,
        u32::from_le_bytes([ //converts 4 bytes into a 32-bit integer using little endian(least significant byte) byte order.
            self.ram[addr],
            self.ram[addr+1],
            self.ram[addr+2],
            self.ram[addr+3],
        ])
    }
    pub fn write_u32(&mut self, addr: u64, value: u32){
        let addr = addr as usize;
        let bytes = value.to_le_bytes(); // convert the u32 into an array of 4 bytes in little - endian order, lets say i store 0x12345678, then adress 0: 0x78, then 1:0x56, then 2: 0x34, and then 3 will be 0x12, thats how x86 works. at least i have been told so.
        self.ram[addr] = bytes[0];
        self.ram[addr+1] = bytes[1];
        self.ram[addr+2] = bytes[2];
        self.ram[addr+3] = bytes[3];
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_read_write_byte(){
        let mut mem = Memory::new(1024);
        mem.write_u8(0, 0x42);
        assert_eq!(mem.read_u8(0), 0x42);
    }
    #[test]
    fn test_read_write_u32(){
        let mut mem = Memory::new(1024);
        mem.write_u32(0, 0x12345678);
        assert_eq!(mem.read_u32(0), 0x12345678);
    }
}