use colored::Colorize;

use crate::cpu::{self, Register};

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
        Opcode::new(0x0, 0x0, "HLT", RefCell::new(move |cpu, argument_1, argument_2| {
            println!("HLT (0x0, 0x0) executed.");
            println!("Cpu stopped.");
            exit(0);
        })),
        Opcode::new(0x0, 0xFFFFFFFF, "NOP", RefCell::new(move |cpu, argument_1, argument_2| {})),
        Opcode::new(0x1, 0x1, "MOV", RefCell::new(move |cpu, argument_1, argument_2| {
            cpu.write_register(argument_1, cpu.read_register(argument_2));
            cpu.update_flags(cpu.read_register(argument_1), cpu.read_register(argument_2));
        })),
        Opcode::new(0x2, 0x1, "MOV", RefCell::new(move |cpu, argument_1, argument_2| {
            cpu.write_register(argument_1, argument_2);
            cpu.update_flags(cpu.read_register(argument_1), argument_2);
        })),
        Opcode::new(0x3, 0x0, "JMP", RefCell::new(move |cpu, argument_1, argument_2| {
            cpu.program_counter = cpu.program_counter.wrapping_add(argument_1 - 3);
        })),
    ];
    let mut map: HashMap<u64, Opcode> = HashMap::new();
    for opcode in opcodes {
        print!("{} (0x{}, 0x{})...", opcode.mnemonic, opcode.opcode_type, opcode.opcode);
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