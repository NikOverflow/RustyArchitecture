use colored::Colorize;

use crate::cpu::{self, Register, Flags};

use std::{collections::HashMap, cell::{RefCell}, process::exit};

#[derive(Clone)]
pub struct Opcode {
    pub opcode_type: u32,
    pub opcode: u32,
    pub mnemonic: &'static str,
    pub callback: RefCell<fn(&mut cpu::CPU, u64, u64)>,
}

impl Opcode {
    fn new(opcode_type: u32, opcode: u32, mnemonic: &'static str, callback: RefCell<fn(&mut cpu::CPU, u64, u64)>) -> Self {
        return Opcode {
            opcode_type: opcode_type,
            opcode: opcode,
            mnemonic: mnemonic,
            callback: callback,
        }
    }
    pub fn execute(self, cpu: &mut cpu::CPU, argument_1: u64, argument_2: u64) {
        (self.callback.borrow_mut())(cpu, argument_1, argument_2);
    }
}
#[allow(unused_variables)]
pub fn initialize() -> HashMap<u64, Opcode> {
    println!("Initialize Opcodes...");
    let opcodes: Vec<Opcode> = vec![
        /*Opcode::new(0x2, 0x1, "MOV", RefCell::new(move |cpu, argument_1, argument_2| {
            cpu.write_register(argument_1, cpu.read_register(argument_2));
            cpu.update_flags(cpu.read_register(argument_1), cpu.read_register(argument_2));
        })),
        Opcode::new(0x3, 0x1, "MOV", RefCell::new(move |cpu, argument_1, argument_2| {
            cpu.write_register(argument_1, argument_2);
            cpu.update_flags(cpu.read_register(argument_1), argument_2);
        })),
        
        Opcode::new(0x4, 0x2, "JMP", RefCell::new(move |cpu, argument_1, argument_2| {
            cpu.program_counter = cpu.program_counter.wrapping_add(argument_1 - 3);
        })),*/


        /* JUMP Instructions */
        Opcode::new(0x04, 0x00, "JMP", RefCell::new(move |cpu, argument_1, argument_2| {
            cpu.program_counter = cpu.program_counter.wrapping_add(argument_1 - 3);
        })),
        Opcode::new(0x04, 0x01, "JE", RefCell::new(move |cpu, argument_1, argument_2| {
            if cpu.flags.contains(Flags::ZERO) {
                cpu.program_counter = cpu.program_counter.wrapping_add(argument_1 - 3);
            }
        })),
        Opcode::new(0x04, 0x02, "JNE", RefCell::new(move |cpu, argument_1, argument_2| {
            if !cpu.flags.contains(Flags::ZERO) {
                cpu.program_counter = cpu.program_counter.wrapping_add(argument_1 - 3);
            }
        })),
        Opcode::new(0x04, 0x03, "JG", RefCell::new(move |cpu, argument_1, argument_2| {
            if !cpu.flags.contains(Flags::SIGN) {
                cpu.program_counter = cpu.program_counter.wrapping_add(argument_1 - 3);
            }
        })),
        Opcode::new(0x04, 0x04, "JGE", RefCell::new(move |cpu, argument_1, argument_2| {
            if !cpu.flags.contains(Flags::SIGN) || cpu.flags.contains(Flags::ZERO) {
                cpu.program_counter = cpu.program_counter.wrapping_add(argument_1 - 3);
            }
        })),
        Opcode::new(0x04, 0x05, "JL", RefCell::new(move |cpu, argument_1, argument_2| {
            if cpu.flags.contains(Flags::SIGN) {
                cpu.program_counter = cpu.program_counter.wrapping_add(argument_1 - 3);
            }
        })),
        Opcode::new(0x04, 0x06, "JLE", RefCell::new(move |cpu, argument_1, argument_2| {
            if cpu.flags.contains(Flags::SIGN) || cpu.flags.contains(Flags::ZERO) {
                cpu.program_counter = cpu.program_counter.wrapping_add(argument_1 - 3);
            }
        })),
        Opcode::new(0x04, 0x07, "JO", RefCell::new(move |cpu, argument_1, argument_2| {
            if cpu.flags.contains(Flags::OVERFLOW) {
                cpu.program_counter = cpu.program_counter.wrapping_add(argument_1 - 3);
            }
        })),
        Opcode::new(0x04, 0x08, "JNO", RefCell::new(move |cpu, argument_1, argument_2| {
            if !cpu.flags.contains(Flags::OVERFLOW) {
                cpu.program_counter = cpu.program_counter.wrapping_add(argument_1 - 3);
            }
        })),
        Opcode::new(0x04, 0x09, "JC", RefCell::new(move |cpu, argument_1, argument_2| {
            if cpu.flags.contains(Flags::CARRY) {
                cpu.program_counter = cpu.program_counter.wrapping_add(argument_1 - 3);
            }
        })),
        Opcode::new(0x04, 0x0A, "JNC", RefCell::new(move |cpu, argument_1, argument_2| {
            if !cpu.flags.contains(Flags::CARRY) {
                cpu.program_counter = cpu.program_counter.wrapping_add(argument_1 - 3);
            }
        })),

        /* Arithmetic Instructions */
        Opcode::new(0x02, 0x00, "ADD", RefCell::new(move |cpu, argument_1, argument_2| {
            let (result, overflowed) = cpu.read_register(argument_2).overflowing_add(cpu.read_register(argument_1));
            cpu.flags.set(Flags::ZERO, result == 0);
            cpu.flags.set(Flags::SIGN, result >> 63 == 1);
            //TODO: Carry Flag Check.
            cpu.flags.set(cpu::Flags::OVERFLOW, overflowed);
            cpu.write_register(argument_1, result);
        })),
        Opcode::new(0x03, 0x00, "ADD", RefCell::new(move |cpu, argument_1, argument_2| {
            let (result, overflowed) = argument_2.overflowing_add(cpu.read_register(argument_1));
            cpu.flags.set(Flags::ZERO, result == 0);
            cpu.flags.set(Flags::SIGN, result >> 63 == 1);
            //TODO: Carry Flag Check.
            cpu.flags.set(cpu::Flags::OVERFLOW, overflowed);
            cpu.write_register(argument_1, result);
        })),

        //TODO ADC.

        Opcode::new(0x02, 0x02, "SUB", RefCell::new(move |cpu, argument_1, argument_2| {
            let (result, overflowed) = cpu.read_register(argument_2).overflowing_sub(cpu.read_register(argument_1));
            cpu.flags.set(Flags::ZERO, result == 0);
            cpu.flags.set(Flags::SIGN, result >> 63 == 1);
            //TODO: Carry Flag Check.
            cpu.flags.set(cpu::Flags::OVERFLOW, overflowed);
            cpu.write_register(argument_1, result);
        })),
        Opcode::new(0x03, 0x02, "SUB", RefCell::new(move |cpu, argument_1, argument_2| {
            let (result, overflowed) = argument_2.overflowing_sub(cpu.read_register(argument_1));
            cpu.flags.set(Flags::ZERO, result == 0);
            cpu.flags.set(Flags::SIGN, result >> 63 == 1);
            //TODO: Carry Flag Check.
            cpu.flags.set(cpu::Flags::OVERFLOW, overflowed);
            cpu.write_register(argument_1, result);
        })),

        //TODO Mul.

        //TODO IMul.

        //TODO Div

        //TODO IDiv

        //TODO SAR

        /* Logical Instructions */

        //TODO AND

        //TODO OR

        //TODO NOT

        //TODO XOR

        //TODO SHL

        //TODO SHR

        /* Other Instructions */

        Opcode::new(0x0, 0x0, "HLT", RefCell::new(move |cpu, argument_1, argument_2| {
            println!("HLT (0x0, 0x0) executed.");
            println!("Cpu stopped.");
            exit(0);
        })),

        Opcode::new(0x00, 0x01, "NOP", RefCell::new(move |cpu, argument_1, argument_2| {})),

        Opcode::new(0x02, 0x0D, "MOV", RefCell::new(move |cpu, argument_1, argument_2| {
            let result: u64 = cpu.read_register(argument_2);
            cpu.flags.set(Flags::ZERO, result == 0);
            cpu.flags.set(Flags::SIGN, result >> 63 == 1);
            //TODO: Carry Flag Check.
            //TODO: Overflow Flag Check.
            cpu.write_register(argument_1, result);
        })),
        Opcode::new(0x03, 0x0D, "MOV", RefCell::new(move |cpu, argument_1, argument_2| {
            cpu.flags.set(Flags::ZERO, argument_2 == 0);
            cpu.flags.set(Flags::SIGN, argument_2 >> 63 == 1);
            //TODO: Carry Flag Check.
            //TODO: Overflow Flag Check.
            cpu.write_register(argument_1, argument_2);
        })),

        Opcode::new(0x02, 0x0E, "CMP", RefCell::new(move |cpu, argument_1, argument_2| {
            let (result, overflowed) = cpu.read_register(argument_2).overflowing_sub(cpu.read_register(argument_1));
            cpu.flags.set(Flags::ZERO, result == 0);
            cpu.flags.set(Flags::SIGN, result >> 63 == 1);
            //TODO: Carry Flag Check.
            cpu.flags.set(cpu::Flags::OVERFLOW, overflowed);
        })),
        Opcode::new(0x03, 0x0E, "CMP", RefCell::new(move |cpu, argument_1, argument_2| {
            let (result, overflowed) = argument_2.overflowing_sub(cpu.read_register(argument_1));
            cpu.flags.set(Flags::ZERO, result == 0);
            cpu.flags.set(Flags::SIGN, result >> 63 == 1);
            //TODO: Carry Flag Check.
            cpu.flags.set(cpu::Flags::OVERFLOW, overflowed);
        })),
        Opcode::new(0x05, 0x0E, "CMP", RefCell::new(move |cpu, argument_1, argument_2| {
            let (result, overflowed) = argument_2.overflowing_sub(argument_1);
            cpu.flags.set(Flags::ZERO, result == 0);
            cpu.flags.set(Flags::SIGN, result >> 63 == 1);
            //TODO: Carry Flag Check.
            cpu.flags.set(cpu::Flags::OVERFLOW, overflowed);
        })),

        //TODO PUSH

        //TODO POP
    ];
    let mut map: HashMap<u64, Opcode> = HashMap::new();
    for opcode in opcodes {
        print!("{} ({:#04X}, {:#04X})...", opcode.mnemonic, opcode.opcode_type, opcode.opcode);
        let complete_opcode: u64 = ((opcode.opcode_type as u64) << 32) | (opcode.opcode as u64);
        map.insert(complete_opcode, opcode);
        println!(" {}", "✔️".green());
    }
    println!("Opcodes initialized.");
    return map;
}
pub fn get_opcode<'a>(instruction_set: &'a HashMap<u64, Opcode>, opcode: u64) -> Option<&'a Opcode> {
    return instruction_set.get(&opcode);
}
pub fn get_u64_opcode(opcode_type: u32, opcode: u32) -> u64 {
    return ((opcode_type as u64) << 32) | (opcode as u64);
}