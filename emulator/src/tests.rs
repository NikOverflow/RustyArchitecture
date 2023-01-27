use crate::{cpu::{self, Register}, opcodes};
#[test]
fn test_mov_instructions() {
    std::thread::Builder::new().stack_size(5000000).spawn(||{
        let mut cpu: cpu::CPU = cpu::CPU::new();
        cpu.execute_opcode(opcodes::MOVI, 0x0, 0xDEADBEEF);
        assert_eq!(cpu.read_register(0x0), 0xDEADBEEF);
        cpu.execute_opcode(opcodes::MOV, 0x1, 0x0);
        assert_eq!(cpu.read_register(0x1), 0xDEADBEEF);
    }).unwrap().join().unwrap();
}