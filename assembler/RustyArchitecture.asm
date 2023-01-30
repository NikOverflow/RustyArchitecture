#subruledef Registers {
    %reg{rnum: u7} => {
        assert(rnum > 0)
        assert(rnum < 65)
        rnum - 1
    }
    %io => 0x40`64
}

#subruledef JumpInstructions {
    jmp => 0x00
    je  => 0x01
    jne => 0x02
    jg  => 0x03
    jge => 0x04
    jl  => 0x05
    jle => 0x06
    jo  => 0x07
    jno => 0x08
    jc  => 0x09
    jnc => 0x0A
}
#subruledef OpcodeTypeNothing {
    hlt => 0x00
    nop => 0x01
}
#subruledef OpcodeTypeRegister {
    not  => 0x00
    push => 0x0B
    pop  => 0x0C
}
#subruledef OpcodeTypeRegisterRegister {
    add  => 0x00
    adc  => 0x01
    sub  => 0x02
    mul  => 0x03
    imul => 0x04
    div  => 0x05
    idiv => 0x06
    sar  => 0x07
    and  => 0x08
    or   => 0x09
    xor  => 0x0A
    shl  => 0x0B
    shr  => 0x0C
    mov  => 0x0D
    cmp  => 0x0E
}
#subruledef OpcodeTypeRegisterImmediate {
    add  => 0x00
    adc  => 0x01
    sub  => 0x02
    mul  => 0x03
    imul => 0x04
    div  => 0x05
    idiv => 0x06
    sar  => 0x07
    and  => 0x08
    or   => 0x09
    xor  => 0x0A
    shl  => 0x0B
    shr  => 0x0C
    mov  => 0x0D
    cmp  => 0x0E
}
#subruledef OpcodeTypeImmediate {
    push => 0x0B
    pop  => 0x0C
}
#subruledef OpcodeTypeImmediateImmediate {
    cmp => 0x0E
}

#ruledef {
    {opcode: JumpInstructions} {address: i64} => {
        reladdr = (address - $) / 8
        0x04`32 @ opcode`32 @ reladdr`64 @ 0x00`64
    }
    {opcode: OpcodeTypeNothing} => 0x00`32 @ opcode`32 @ 0x00`64 @ 0x00`64
    {opcode: OpcodeTypeRegister} {reg: Registers} => 0x01`32 @ opcode`32 @ reg`64 @ 0x00`64
    {opcode: OpcodeTypeRegisterRegister} {reg: Registers}, {value: Registers} => 0x02`32 @ opcode`32 @ reg`64 @ value`64
    {opcode: OpcodeTypeRegisterImmediate} {reg: Registers}, {value: i64} => 0x03`32 @ opcode`32 @ reg`64 @ value`64
    {opcode: OpcodeTypeImmediate} {value: i64} => 0x04`32 @ opcode`32 @ value`64 @ 0x00`64
    {opcode: OpcodeTypeImmediateImmediate} {int: i64}, {value: i64} => 0x05`32 @ opcode`32 @ int`64 @ value`64
}