use crate::opcodes;

use std::{collections::HashMap, process::exit};

use bitflags::bitflags;

bitflags! {
    pub struct Flags: u8 {
        const ZERO     = 0b00000001;
        const NEGATIVE = 0b00000010;
        const CARRY    = 0b00000100;
        const OVERFLOW = 0b00001000;
    }
}

pub struct CPU {
    pub registers: [u64; 4],
    pub flags: Flags,
    pub program_counter: u64,
    memory: [u64; 131072],
    instruction_set: HashMap<u64, opcodes::Opcode>,
}

impl CPU {
    pub fn new() -> Self {
        println!("Initialize Cpu...");
        let instruction_set: HashMap<u64, opcodes::Opcode> = opcodes::initialize();
        //self.bus = bus::initialize();
        println!("Cpu initialized.");
        CPU {
            registers: [0; 4],
            program_counter: 0,
            flags: Flags::empty(),
            memory: [0; 131072],
            instruction_set: instruction_set,
        }
    }
    pub fn load(&mut self, program: Vec<u64>) {
        println!("Loading program...");
        self.memory[..program.len()].copy_from_slice(&program[..]);
        println!("Program loaded.");
        println!("Instructions: {:?}", program);
    }
    /*pub fn run<F>(&mut self, mut callback: F) where F: FnMut(&mut CPU) {
        callback(self);
    }*/
    pub fn run(&mut self) {
        loop {
            self.execute_instruction();
            println!("All Registers: {:?}", self.registers);
            ::std::thread::sleep(std::time::Duration::new(0, 70_000));
            //::std::thread::sleep(std::time::Duration::new(0, 90000_000));
        }
    }
    pub fn reset(&mut self) {
        for register in self.registers.iter_mut() {
            *register = 0;
        }
        self.flags = Flags::empty();
        self.program_counter = 0;
    }

    pub fn update_flags(&mut self, old_value: u64, new_value: u64) {
        self.flags.set(Flags::ZERO, new_value == 0);
        self.flags.set(Flags::NEGATIVE, new_value >> 63 == 1);
        //TODO: Add Carry Flag check.
        //TODO; Add Overflow Flag check.
    }

    pub fn execute_opcode(&mut self, opcode: u64, argument_1: u64, argument_2: u64) {
        let complete_opcode: Option<&opcodes::Opcode> = opcodes::get_opcode(&self.instruction_set, opcode);
        match complete_opcode {
            Some(opcode) => {
                let cloned_opcode = opcode.clone();
                let mnemonic = cloned_opcode.mnemonic;
                let opcode_type = cloned_opcode.opcode_type;
                let opcode = cloned_opcode.opcode;
                cloned_opcode.execute(self, argument_1, argument_2);
                println!("{} (0x{}, 0x{}) executed.", mnemonic, opcode_type, opcode);
            },
            None => {
                println!("Instruction ({}) does not exist!", opcode);
                exit(1);
            }
        }
    }
    /*pub fn peek_instruction(&mut self) -> (u32, u32, u64, u64) {
        let complete_opcode: u64 = self.memory_read(self.program_counter);
        let opcode_type: u32 = (complete_opcode >> 32) as u32;
        let opcode: u32 = (complete_opcode & 0xFFFFFFFF) as u32;
        let argument_1: u64 = self.memory_read(self.program_counter + 1);
        let argument_2: u64 = self.memory_read(self.program_counter + 2);
        return (opcode_type, opcode, argument_1, argument_2)
    }
    pub fn read_instruction(&mut self) -> (u32, u32, u64, u64) {
        let instruction = self.peek_instruction();
        self.next();
        return instruction;
    }
    pub fn next(&mut self) {
        self.program_counter += 3;
    }*/
    pub fn execute_instruction(&mut self) {
        let opcode: u64 = self.memory_read(self.program_counter);
        let argument_1: u64 = self.memory_read(self.program_counter + 1);
        let argument_2: u64 = self.memory_read(self.program_counter + 2);
        self.execute_opcode(opcode, argument_1, argument_2);
        self.program_counter += 3;
    }
}

pub trait Memory {
    fn memory_read(&self, address: u64) -> u64;
    fn memory_write(&mut self, address: u64, data: u64);
}
impl Memory for CPU {
    fn memory_read(&self, address: u64) -> u64 {
        return self.memory[address as usize];
    }
    fn memory_write(&mut self, address: u64, data: u64) {
        self.memory[address as usize] = data;
    }
}
pub trait Register {
    fn read_register(&self, address: u64) -> u64;
    fn write_register(&mut self, address: u64, data: u64);
}
impl Register for CPU {
    fn read_register(&self, address: u64) -> u64 {
        return self.registers[address as usize];
    }
    fn write_register(&mut self, address: u64, data: u64) {
        self.registers[address as usize] = data;
    }
}