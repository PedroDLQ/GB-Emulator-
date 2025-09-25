macro_rules! get_set {
    
    ($reg:ident, $get_name:ident, $set_name:ident, $size:ty) => {

        pub fn $get_name(&self) -> $size {

            self.$reg
        }

        pub fn $set_name(&mut self, val: $size) {

            self.$reg = val;
        }
    };
}

macro_rules! get_set_comp { // For merged BC, CD, HL registers

    ($reg1:ident, reg2:ident, $get_name:ident, $set_name:ident) => {

        pub fn $get_name(&self) -> u16 {

            (self.$reg1 as u16) << 8 | self.$reg2 as u16
        }

        pub fn $set_name(&mut self, val: u16) {

            self.$reg1 = (val >> 8) as u8;
            self.$reg2 = val as u8; 
        }
    }
}

pub struct Registers {

    a: u8,
    b: u8,
    c: u8, 
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
} 

impl Registers {

    pub fn new() -> Registers {

        Registers {

            a: 0,
            b: 0,
            c: 0, 
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
        }
    }

    // Base register get/set's
    get_set!(a, get_a, set_a, u8);
    get_set!(b, get_b, set_b, u8);
    get_set!(c, get_c, set_c, u8);
    get_set!(d, get_d, set_d, u8);
    get_set!(e, get_e, set_e, u8);
    get_set!(h, get_h, set_h, u8);
    get_set!(l, get_l, set_l, u8);
    get_set!(sp, get_sp, set_sp, u16);
    get_set!(pc, get_pc, set_pc, u16);

    // Composed 16bit register get/set's
    get_set_comp!(b, c, get_bc, set_bc);
    get_set_comp!(d, e, get_de, set_de);
    get_set_comp!(h, l, get_hl, set_hl);

    // F special behaviour (flag reg)
    pub fn get_f(&self) -> u8 {

        self.f
    }

    pub fn set_f(&mut self, val: u8) {

        self.f = val & 0xF0; // Masking the last 4 bits
    }
}
