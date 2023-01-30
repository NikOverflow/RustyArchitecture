use crate::{cpu::{self, Register}, opcodes};
#[test]
fn test_mov_instruction() {
    std::thread::Builder::new().stack_size(5000000).spawn(||{
        let mut cpu: cpu::CPU = cpu::CPU::new(true);
        cpu.execute_opcode(opcodes::get_u64_opcode(0x03, 0x0D), 0x0, 0xDEADBEEF);
        assert_eq!(cpu.read_register(0x0), 0xDEADBEEF);
        cpu.execute_opcode(opcodes::get_u64_opcode(0x02, 0x0D), 0x1, 0x0);
        assert_eq!(cpu.read_register(0x1), 0xDEADBEEF);
    }).unwrap().join().unwrap();
}