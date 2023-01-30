Registers:
- pc -> Program Counter
- sp -> Stack Pointer

# Specifications

## Registers

Coming Soon!

## Flags

| Flag | Mask | Abbreviation |
|------|------|--------------|
| Zero | 0b0001 | ZF |
| Sign | 0b0010 | SF |
| Carry | 0b0100 | CF |
| Overflow | 0b1000 | OF |

## Opcode Types
> The opcode type tells which arguments exist.

| Type | Meaning |
|------|---------|
| 0x00 | Nothing |
| 0x01 | Register |
| 0x02 | dest. Register, src. Register |
| 0x03 | dest. Register, Immediate |
| 0x04 | Immediate |
| 0x05 | Immediate, Immediate |

## Opcodes
> The complete opcode is 64 bits long and consists of the opcode type for the upper 32 bits and the opcode itself for the lower 32 bits.

### Jump Opcodes

| Opcode | Mnemonic | Opcode Types |
|--------|----------|--------------|
| 0x00 | JMP | 0x04 |
| 0x01 | JE | 0x04 |
| 0x02 | JNE | 0x04 |
| 0x03 | JG | 0x04 |
| 0x04 | JGE | 0x04 |
| 0x05 | JL | 0x04 |
| 0x06 | JLE | 0x04 |
| 0x07 | JO | 0x04 |
| 0x08 | JNO | 0x04 |
| 0x09 | JC | 0x04 |
| 0x0A | JNC | 0x04 |

### Arithmetic Opcodes

| Opcode | Mnemonic | Opcode Types |
|--------|----------|--------------|
| 0x00 | ADD | 0x02 or 0x03 |
| 0x01 | ADC | 0x02 or 0x03 |
| 0x02 | SUB | 0x02 or 0x03 |
| 0x03 | MUL | 0x02 or 0x03 |
| 0x04 | IMUL | 0x02 or 0x03 |
| 0x05 | DIV | 0x02 or 0x03 |
| 0x06 | IDIV | 0x02 or 0x03 |
| 0x07 | SAR | 0x02 or 0x03 |

### Logical Opcodes

| Opcode | Mnemonic | Opcode Types |
|--------|----------|--------------|
| 0x08 | AND | 0x02 or 0x03 |
| 0x09 | OR | 0x02 or 0x03 |
| 0x00 | NOT | 0x01 |
| 0x0A | XOR | 0x02 or 0x03 |
| 0x0B | SHL | 0x02 or 0x03 |
| 0x0C | SHR | 0x02 or 0x03 |

### Other Opcodes

| Opcode | Mnemonic | Opcode Types |
|--------|----------|--------------|
| 0x00 | HLT | 0x00 |
| 0x01 | NOP | 0x00 |
| 0x0D | MOV | 0x02 or 0x03 |
| 0x0E | CMP | 0x02, 0x03 or 0x05 |
| 0x0B | PUSH | 0x01 or 0x04 |
| 0x0C | POP | 0x01 or 0x04 |

## Instructions
> An instruction is 3x 64-bit numbers long. If an argument is not present, it is represented as 0.<br>
> The instruction is composed of the whole opcode and then the 2 arguments.

### Assembler
> Assembler instructions look like this:<br>
> INSTRUCTION<br>
> INSTRUCTION ARGUMENT<br>
> INSTRUCTION ARGUMENT 1, ARGUMENT 2

<!--| Instruction |
|-------------|
| jmp |
| je |
| jne |
| jz |
| jnz |
| jg |
| jge |
| jl |
| jle |
| jo |
| jno |
| add |
| and |
| or |
| not |
| xor |
| push |
| pop |
| hlt |
| nop |
| mov |
| cmp |-->