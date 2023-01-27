pub mod cpu;
pub mod opcodes;

use std::fs;
use std::io::Read;

fn main() {
    println!("Starting RustyArchitecture Emulator.");
    let mut cpu: cpu::CPU = cpu::CPU::new();
    let program: Vec<u64> = get_instructions_from_file(&String::from("jmp.bin"));
    cpu.load(program);
    cpu.run();
}

fn get_instructions_from_file(filename: &String) -> Vec<u64> {
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
    return instructions;
}