use crate::intel_8085::intel_8085::Cpu8085;

//#[allow(overflowing_literals)]

fn execute_mov(cpu: &mut Cpu8085, opcode: u8, data_immediate: Option<i8>){
    match opcode {
        0x40..=0x45 | 0x47..=0x4D | 0x4F..=0x55 | 0x57..=0x5d | 0x5F..=0x65 | 0x67..=0x6D | 0x6F | 0x78..=0x7D | 0x7F => register_to_register_mov(cpu, opcode),
        0x46 | 0x4E | 0x56 | 0x5E | 0x66 | 0x6E | 0x7E => memory_to_register_mov(cpu, opcode),
        0x70..=0x75 | 0x77 => register_to_register_mov(cpu, opcode),
        0x3E | 0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E => data_to_register_mov(cpu, opcode, data_immediate.unwrap()),
        0x35 => data_to_memory_mov(cpu, opcode, data_immediate.unwrap()),
        _ => panic!("AAAAAA"),
    }
}
fn data_to_memory_mov(cpu: &mut Cpu8085, opcode: u8, data_immediate: i8){
    let address: u16  = (cpu.get_gp_reg('h') as u16) << 8 | cpu.get_gp_reg('l') as u16;
    cpu.write_memory(address, data_immediate);
}
fn data_to_register_mov(cpu: &mut Cpu8085, opcode: u8, data_immediate: i8){
    match opcode {
        0x3E => cpu.set_gp_reg('a', data_immediate),
        0x06 => cpu.set_gp_reg('b', data_immediate),
        0x0E => cpu.set_gp_reg('c', data_immediate),
        0x16 => cpu.set_gp_reg('d', data_immediate),
        0x1E => cpu.set_gp_reg('e', data_immediate),
        0x26 => cpu.set_gp_reg('h', data_immediate),
        0x2E => cpu.set_gp_reg('l', data_immediate),
        _ => panic!("AAAAAA"),
    }
}
fn register_to_memory_mov(cpu: &mut Cpu8085, opcode: u8){
    let address: u16  = (cpu.get_gp_reg('h') as u16) << 8 | cpu.get_gp_reg('l') as u16;
    match opcode {
        0x70 => cpu.write_memory(address, cpu.get_gp_reg('b')),
        0x71 => cpu.write_memory(address, cpu.get_gp_reg('c')),
        0x72 => cpu.write_memory(address, cpu.get_gp_reg('d')),
        0x73 => cpu.write_memory(address, cpu.get_gp_reg('e')),
        0x74 => cpu.write_memory(address, cpu.get_gp_reg('h')),
        0x75 => cpu.write_memory(address, cpu.get_gp_reg('l')),
        0x77 => cpu.write_memory(address, cpu.get_gp_reg('a')),
        _ => panic!("AAAAAAAAAAAAAAAA"),
    }
}
fn memory_to_register_mov(cpu: &mut Cpu8085, opcode: u8){
    let address: u16  = (cpu.get_gp_reg('h') as u16) << 8 | cpu.get_gp_reg('l') as u16;
    match opcode {
        0x46 => cpu.set_gp_reg('b', cpu.read_memory(address)),
        0x4E => cpu.set_gp_reg('c', cpu.read_memory(address)),
        0x56 => cpu.set_gp_reg('d', cpu.read_memory(address)),
        0x5E => cpu.set_gp_reg('e', cpu.read_memory(address)),
        0x66 => cpu.set_gp_reg('h', cpu.read_memory(address)),
        0x6E => cpu.set_gp_reg('l', cpu.read_memory(address)),
        0x7E => cpu.set_gp_reg('b', cpu.read_memory(address)),
        _ => panic!("AAAAAAAAAAA"),
    }
}
fn register_to_register_mov(cpu: &mut Cpu8085, opcode: u8){
    match opcode{
        0x40..=0x45 => {
            match opcode {
                0x40 => cpu.set_gp_reg('b', cpu.get_gp_reg('b')),
                0x41 => cpu.set_gp_reg('b', cpu.get_gp_reg('c')),
                0x42 => cpu.set_gp_reg('b', cpu.get_gp_reg('d')),
                0x43 => cpu.set_gp_reg('b', cpu.get_gp_reg('e')),
                0x44 => cpu.set_gp_reg('b', cpu.get_gp_reg('h')),
                0x45 => cpu.set_gp_reg('b', cpu.get_gp_reg('l')),
                _ => panic!("AAAAAA"),
            }
        },
        0x47..=0x4D => {
                match opcode {
                    0x47 => cpu.set_gp_reg('b', cpu.get_gp_reg('a')),
                    0x48 => cpu.set_gp_reg('c', cpu.get_gp_reg('b')),
                    0x49 => cpu.set_gp_reg('c', cpu.get_gp_reg('c')),
                    0x4A => cpu.set_gp_reg('c', cpu.get_gp_reg('d')),
                    0x4B => cpu.set_gp_reg('c', cpu.get_gp_reg('e')),
                    0x4C => cpu.set_gp_reg('c', cpu.get_gp_reg('h')),
                    0x4D => cpu.set_gp_reg('c', cpu.get_gp_reg('l')),
                    _ => panic!("AAAAAA"),
                }
        },
        0x4F..=0x55 => {
            match opcode {
                0x4F => cpu.set_gp_reg('c', cpu.get_gp_reg('a')),
                0x50 => cpu.set_gp_reg('d', cpu.get_gp_reg('b')),
                0x51 => cpu.set_gp_reg('d', cpu.get_gp_reg('c')),
                0x52 => cpu.set_gp_reg('d', cpu.get_gp_reg('d')),
                0x53 => cpu.set_gp_reg('d', cpu.get_gp_reg('e')),
                0x54 => cpu.set_gp_reg('d', cpu.get_gp_reg('h')),
                0x55 => cpu.set_gp_reg('d', cpu.get_gp_reg('l')),
                _ => panic!("AAAAAA"),
            }
        },
        0x57..=0x5d => {
            match opcode {
                0x57 => cpu.set_gp_reg('d', cpu.get_gp_reg('a')),
                0x58 => cpu.set_gp_reg('e', cpu.get_gp_reg('b')),
                0x59 => cpu.set_gp_reg('e', cpu.get_gp_reg('c')),
                0x5A => cpu.set_gp_reg('e', cpu.get_gp_reg('d')),
                0x5B => cpu.set_gp_reg('e', cpu.get_gp_reg('e')),
                0x5C => cpu.set_gp_reg('e', cpu.get_gp_reg('h')),
                0x5D => cpu.set_gp_reg('e', cpu.get_gp_reg('l')),
                _ => panic!("AAAAAA"),
            }
        },
        0x5F..=0x65 => {
            match opcode {
                0x5F => cpu.set_gp_reg('e', cpu.get_gp_reg('a')),
                0x60 => cpu.set_gp_reg('h', cpu.get_gp_reg('b')),
                0x61 => cpu.set_gp_reg('h', cpu.get_gp_reg('c')),
                0x62 => cpu.set_gp_reg('h', cpu.get_gp_reg('d')),
                0x63 => cpu.set_gp_reg('h', cpu.get_gp_reg('e')),
                0x64 => cpu.set_gp_reg('h', cpu.get_gp_reg('h')),
                0x65 => cpu.set_gp_reg('h', cpu.get_gp_reg('l')),
                _ => panic!("AAAAAA"),
            }
        },
        0x67..=0x6D => {
            match opcode {
                0x67 => cpu.set_gp_reg('h', cpu.get_gp_reg('a')),
                0x68 => cpu.set_gp_reg('l', cpu.get_gp_reg('b')),
                0x69 => cpu.set_gp_reg('l', cpu.get_gp_reg('c')),
                0x6A => cpu.set_gp_reg('l', cpu.get_gp_reg('d')),
                0x6B => cpu.set_gp_reg('l', cpu.get_gp_reg('e')),
                0x6C => cpu.set_gp_reg('l', cpu.get_gp_reg('h')),
                0x6D => cpu.set_gp_reg('l', cpu.get_gp_reg('l')),
                _ => panic!("AAAAAA"),
            }
        },
        0x6F => cpu.set_gp_reg('l', cpu.get_gp_reg('a')),

        0x78..=0x7D => {
            match opcode {
                0x78 => cpu.set_gp_reg('a', cpu.get_gp_reg('b')),
                0x79 => cpu.set_gp_reg('a', cpu.get_gp_reg('c')),
                0x7A => cpu.set_gp_reg('a', cpu.get_gp_reg('d')),
                0x7B => cpu.set_gp_reg('a', cpu.get_gp_reg('e')),
                0x7C => cpu.set_gp_reg('a', cpu.get_gp_reg('h')),
                0x7D => cpu.set_gp_reg('a', cpu.get_gp_reg('l')),
                _ => panic!("AAAAAA"),
            }
        },
        0x7F => cpu.set_gp_reg('a', cpu.get_gp_reg('a')),
        _ => panic!("AAAAAAAAAA"),
    }
}