const MAX_ADDR: u16 = 0xFFFF;


pub struct Cpu8085{
    b: i8,
    c: i8,
    d: i8,
    e: i8,
    h: i8,
    l: i8,
    accumulator: i8,    //also called "register A"
    flag_register: u8,  //bit 7 is sign, 6 is zero, 4 is auxiliary carry, 2 is parity status, 0 is carry
    program_counter: u16,
    stack_pointer: u16,
    instruction_register: i8,
    temp_register_1: i8,  //Also called "register W".Not to be used by programmer. Only used by cpu to store intermediate data during computation.
    temp_register_2: i8,  //Also called "register Z".Not to be used by programmer. Only used by cpu to store intermediate data during computation.

    memory: [i8; MAX_ADDR as usize],
}impl Cpu8085 {
    //
    //special
    //
    pub fn initialize() -> Self{
        Cpu8085 { b: 0, c: 0, d: 0, e: 0, h: 0, l: 0, accumulator: 0, flag_register: 0, program_counter: 0, stack_pointer: 0, instruction_register: 0, temp_register_1: 0, temp_register_2: 0, memory: [0; MAX_ADDR as usize] }
    }
    

    //
    //getters
    //
    pub(crate) fn get_gp_reg(&self, register: char) -> i8{
        match register {
            'b' => self.b,
            'c' => self.c,
            'd' => self.d,
            'e' => self.e,
            'h' => self.h,
            'l' => self.l,
            'a' => self.accumulator,
            _ => panic!("You entered an invalid general purpose register. Now you must die.")
        }
    }

    fn get_pc(&self) ->u16{
        self.program_counter
    }
    fn get_sp(&self) -> u16{
        self.stack_pointer
    }
    //
    //setters
    //
    pub(super) fn set_gp_reg(&mut self, register: char, value: i8){
        match register {
            'b' => self.b = value,
            'c' => self.c = value,
            'd' => self.d = value,
            'e' => self.e = value,
            'h' => self.h = value,
            'l' => self.l = value,
            'a' => self.accumulator = value,
            _ => panic!("You entered an invalid general purpose register. Now you must die.")
        }
    }
    
    //
    //flag registers
    //
    pub(super) fn get_flag(&self, flag: &str)-> u8{
        match flag {
            "sign" => (self.flag_register & 0b_1000_0000) >> 7,
            "zero" => (self.flag_register & 0b_0100_0000) >> 6,
            "aux_carry" => (self.flag_register & 0b_0001_0000) >> 4,
            "parity" => (self.flag_register & 0b_0000_0100) >> 2,
            "carry" => self.flag_register & 0b_0000_0001,
            _ => panic!("You entered an invalid flag. Now you must die.")
        }
    }
    
    fn set_flag(&mut self, flag: &str, value: i8){
        match flag {
            "sign" => {
                if value == 0 {
                    self.flag_register &= 0b_0111_1111;
                }else {
                    self.flag_register |= 0b_1000_0000;
                }
            }
            "zero" => {
                if value == 0 {
                    self.flag_register &= 0b_1011_1111;
                }else {
                    self.flag_register |= 0b_0100_0000;
                }
            }
            "aux_carry" => {
                if value == 0 {
                    self.flag_register &= 0b_1110_1111;
                }else {
                    self.flag_register |= 0b_0001_0000;
                }
            }
            "parity" => {
                if value == 0 {
                    self.flag_register &= 0b_1111_1011;
                }else {
                    self.flag_register |= 0b_0000_0100;
                }
            }
            "carry" => {
                if value == 0 {
                    self.flag_register &= 0b_1111_1110;
                }else {
                    self.flag_register |= 0b_0000_0001;
                }
            }
            _ => panic!("You entered an invalid flag. Now you must die.")
        }
    }
    
    //
    //Memory
    //
    pub(super) fn read_memory(&self, address: u16) -> i8{
        self.memory[address as usize]
    }
    pub(super) fn write_memory(&mut self, address: u16, value: i8){
        self.memory[address as usize] = value;
    }

    /* 
    fn execute_instruction(&mut self, opcode: i8) {
        match opcode {
            0x00 => ,
            0x01 => ,
            0x02 => ,
            0x03 => ,
            0x04 => ,
            0x05 => ,
            0x06 => ,
            0x07 => ,
            0x08 => ,
            0x09 => ,
            0x0A => ,
            0x0B => ,
            0x0C => ,
            0x0D => ,
            0x0E => ,
            0x0F => ,
            0x10 => ,
            0x11 => ,
            0x12 => ,
            0x13 => ,
            0x14 => ,
            0x15 => ,
            0x16 => ,
            0x17 => ,
            0x18 => ,
            0x19 => ,
            0x1A => ,
            0x1B => ,
            0x1C => ,
            0x1D => ,
            0x1E => ,
            0x1F => ,
            0x20 => ,
            0x21 => ,
            0x22 => ,
            0x23 => ,
            0x24 => ,
            0x25 => ,
            0x26 => ,
            0x27 => ,
            0x28 => ,
            0x29 => ,
            0x2A => ,
            0x2B => ,
            0x2C => ,
            0x2D => ,
            0x2E => ,
            0x2F => ,
            0x30 => ,
            0x31 => ,
            0x32 => ,
            0x33 => ,
            0x34 => ,
            0x35 => ,
            0x36 => ,
            0x37 => ,
            0x38 => ,
            0x39 => ,
            0x3A => ,
            0x3B => ,
            0x3C => ,
            0x3D => ,
            0x3E => ,
            0x3F => ,
            0x40..=0x7F => ,
            0x80 => ,
            0x81 => ,
            0x82 => ,
            0x83 => ,
            0x84 => ,
            0x85 => ,
            0x86 => ,
            0x87 => ,
            0x88 => ,
            0x89 => ,
            0x8A => ,
            0x8B => ,
            0x8C => ,
            0x8D => ,
            0x8E => ,
            0x8F => ,
            0x90 => ,
            0x91 => ,
            0x92 => ,
            0x93 => ,
            0x94 => ,
            0x95 => ,
            0x96 => ,
            0x97 => ,
            0x98 => ,
            0x99 => ,
            0x9A => ,
            0x9B => ,
            0x9C => ,
            0x9D => ,
            0x9E => ,
            0x9F => ,
            0xA0 => ,
            0xA1 => ,
            0xA2 => ,
            0xA3 => ,
            0xA4 => ,
            0xA5 => ,
            0xA6 => ,
            0xA7 => ,
            0xA8 => ,
            0xA9 => ,
            0xAA => ,
            0xAB => ,
            0xAC => ,
            0xAD => ,
            0xAE => ,
            0xAF => ,
            0xB0 => ,
            0xB1 => ,
            0xB2 => ,
            0xB3 => ,
            0xB4 => ,
            0xB5 => ,
            0xB6 => ,
            0xB7 => ,
            0xB8 => ,
            0xB9 => ,
            0xBA => ,
            0xBB => ,
            0xBC => ,
            0xBD => ,
            0xBE => ,
            0xBF => ,
            0xC0 => ,
            0xC1 => ,
            0xC2 => ,
            0xC3 => ,
            0xC4 => ,
            0xC5 => ,
            0xC6 => ,
            0xC7 => ,
            0xC8 => ,
            0xC9 => ,
            0xCA => ,
            0xCB => ,
            0xCC => ,
            0xCD => ,
            0xCE => ,
            0xCF => ,
            0xD0 => ,
            0xD1 => ,
            0xD2 => ,
            0xD3 => ,
            0xD4 => ,
            0xD5 => ,
            0xD6 => ,
            0xD7 => ,
            0xD8 => ,
            0xD9 => ,
            0xDA => ,
            0xDB => ,
            0xDC => ,
            0xDD => ,
            0xDE => ,
            0xDF => ,
            0xE0 => ,
            0xE1 => ,
            0xE2 => ,
            0xE3 => ,
            0xE4 => ,
            0xE5 => ,
            0xE6 => ,
            0xE7 => ,
            0xE8 => ,
            0xE9 => ,
            0xEA => ,
            0xEB => ,
            0xEC => ,
            0xED => ,
            0xEE => ,
            0xEF => ,
            0xF0 => ,
            0xF1 => ,
            0xF2 => ,
            0xF3 => ,
            0xF4 => ,
            0xF5 => ,
            0xF6 => ,
            0xF7 => ,
            0xF8 => ,
            0xF9 => ,
            0xFA => ,
            0xFB => ,
            0xFC => ,
            0xFD => ,
            0xFE => ,
            0xFF => 
        }
    }*/
}

#[test]
fn test_general_purpose_registers() {
    let mut test_cpu = Cpu8085::initialize();
    test_cpu.set_gp_reg('b', 13);
    test_cpu.set_gp_reg('c', 14);
    test_cpu.set_gp_reg('d', 15);
    test_cpu.set_gp_reg('e', 16);
    test_cpu.set_gp_reg('h', 17);
    test_cpu.set_gp_reg('l', 18);
    test_cpu.set_gp_reg('a', 19);

    assert_eq!(test_cpu.get_gp_reg('b'), 13, "register b test has failed!");
    assert_eq!(test_cpu.get_gp_reg('c'), 14, "register c test has failed!");
    assert_eq!(test_cpu.get_gp_reg('d'), 15, "register d test has failed!");
    assert_eq!(test_cpu.get_gp_reg('e'), 16, "register e test has failed!");
    assert_eq!(test_cpu.get_gp_reg('h'), 17, "register h test has failed!");
    assert_eq!(test_cpu.get_gp_reg('l'), 18, "register l test has failed!");
    assert_eq!(test_cpu.get_gp_reg('a'), 19, "register accumulator test has failed!");
}

#[test]
fn test_flag_register(){

    let mut test_cpu = Cpu8085::initialize();

    assert_eq!(test_cpu.get_flag("sign"), 0);
    test_cpu.set_flag("sign", 1);
    assert_eq!(test_cpu.get_flag("sign"), 1);
    test_cpu.set_flag("sign", 0);
    assert_eq!(test_cpu.get_flag("sign"), 0);

    assert_eq!(test_cpu.get_flag("zero"), 0);
    test_cpu.set_flag("zero", 1);
    assert_eq!(test_cpu.get_flag("zero"), 1);
    test_cpu.set_flag("zero", 0);
    assert_eq!(test_cpu.get_flag("zero"), 0);

    assert_eq!(test_cpu.get_flag("aux_carry"), 0);
    test_cpu.set_flag("aux_carry", 1);
    assert_eq!(test_cpu.get_flag("aux_carry"), 1);
    test_cpu.set_flag("aux_carry", 0);
    assert_eq!(test_cpu.get_flag("aux_carry"), 0);

    assert_eq!(test_cpu.get_flag("parity"), 0);
    test_cpu.set_flag("parity", 1);
    assert_eq!(test_cpu.get_flag("parity"), 1);
    test_cpu.set_flag("parity", 0);
    assert_eq!(test_cpu.get_flag("parity"), 0);

    assert_eq!(test_cpu.get_flag("carry"), 0);
    test_cpu.set_flag("carry", 1);
    assert_eq!(test_cpu.get_flag("carry"), 1);
    test_cpu.set_flag("carry", 0);
    assert_eq!(test_cpu.get_flag("carry"), 0);

}

//#[test]

