use crate::GB::registers; // Importing the registers submodule

pub struct Cpu {

    pub registers: registers::Registers,
    pub ime: bool,  // Interrupt Master Enable: intercepting interrupt calls
    pub opcode: u8, // Running instruction Operation code
    pub cycles: u64, // Cycle counter 
}

impl Cpu {

    pub fn new() -> Self {
        
        //TODO
    }

    pub fn fetch_next(&mut self) -> u8 {

        //TODO
    }

    pub fn decode(opcode: u8, cb_opcode: bool) -> instructions::Intrusction {

        //TODO
    }

    pub fn execute_next(&mut self) -> u64 {

        //TODO
    }
}