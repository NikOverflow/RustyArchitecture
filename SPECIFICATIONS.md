Registers:
- pc -> Program Counter
- sp -> Stack Pointer

Flags:
- Z -> Zero
- N -> Negative
- C -> Carry
- V -> Overflow

Opcode Types:
- 0x0 -> Nothing
- 0x1 -> Register, Register
- 0x2 -> Register, Immediate
- 0x3 -> Immediate, Immediate
- 0x4 -> Conditional Jump

Opcodes:
- 0x0 -> HLT (0x0)
- 0x1 -> MOV (0x1, 0x2)

- 0xFFFFFFFF -> NOP (0x0)

Example Instructions:
- 0x0000000100000001 0x0 0x1 (mov %reg1, %reg2)
- 0x0 0x0 0x0 (hlt)
- 0x0000000011111111 0x0 0x0 (nop)