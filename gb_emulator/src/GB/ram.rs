pub struct Ram {

    memory: [u8; 65536], // Max memory size of the Game Boy system (65.536 kB)
}

impl Ram {

    pub fn new() -> self { // Struct constructor

        Ram { memory: [0; 65536] } 
    }

    pub fn read(&self, address: u16) -> u8 {

        self.memory[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u8) {

        self.memory[address as usize] = value;
    }
}
