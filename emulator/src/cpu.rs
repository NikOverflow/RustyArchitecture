use crate::opcodes;

use std::{collections::HashMap, process::exit};
use std::fs;
use std::io::Read;

use bitflags::bitflags;

bitflags! {
    pub struct Flags: u8 {
        const ZERO     = 0b00000001;
        const SIGN = 0b00000010;
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
    verbose: bool,
}

impl CPU {
    pub fn new(verbose: bool) -> Self {
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
            verbose: verbose,
        }
    }
    pub fn load(&mut self, program: Vec<u64>) {
        println!("Loading program...");
        self.memory[..program.len()].copy_from_slice(&program[..]);
        println!("Program loaded.");
        println!("Instructions: {:?}", program);
    }
    pub fn load_from_file(&mut self, filename: &str) {
        let mut instructions: Vec<u64> = vec![];
        let mut file = fs::File::open(&filename).expect("no file found");
        let metadata = fs::metadata(&filename).expect("unable to read metadata");
        let mut buffer: Vec<u8> = vec![0; metadata.len() as usize];
        file.read(&mut buffer).expect("buffer overflow");
        let mut instruction: u64 = 0;
        let mut counter: u8 = 0;
        for i in buffer.iter() {
            let integer: u64 = *i as u64;
            instruction = (instruction << 8) | integer;
            if counter == 7 {
                instructions.push(instruction);
                instruction = 0;
                counter = 0;
            } else { 
                counter += 1;
            }
        }
        self.load(instructions)
    }
    /*pub fn run<F>(&mut self, mut callback: F) where F: FnMut(&mut CPU) {
        callback(self);
    }*/
    pub fn run(&mut self) {
        loop {
            self.execute_instruction();
            //println!("All Registers: {:?}", self.registers);
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

    pub fn update_common_flags(&mut self, value: u64) {
        self.flags.set(Flags::ZERO, value == 0);
        self.flags.set(Flags::SIGN, value >> 63 == 1);
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
                if self.verbose {
                    print!("{} [({:#04x}, {:#04x}), ({}", mnemonic, opcode_type, opcode, mnemonic.to_lowercase());
                    if opcode_type == 0x0 {
                        println!(")] exeuted.");
                    } else if opcode_type == 0x1 || opcode_type == 0x4 {
                        println!(" {})] executed.", argument_1);
                    } else {
                        println!(" {}, {})] executed.", argument_1, argument_2);
                    }
                }
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
        let opcode: u64 = self.read_memory(self.program_counter);
        let argument_1: u64 = self.read_memory(self.program_counter + 1);
        let argument_2: u64 = self.read_memory(self.program_counter + 2);
        self.execute_opcode(opcode, argument_1, argument_2);
        self.program_counter = self.program_counter.wrapping_add(3);
    }
}

pub trait Memory {
    fn read_memory(&self, address: u64) -> u64;
    fn write_memory(&mut self, address: u64, data: u64);
}
impl Memory for CPU {
    fn read_memory(&self, address: u64) -> u64 {
        return self.memory[address as usize];
    }
    fn write_memory(&mut self, address: u64, data: u64) {
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