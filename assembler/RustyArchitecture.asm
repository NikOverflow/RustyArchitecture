#subruledef Registers {
    %reg{rnum: u7} => {
        assert(rnum > 0)
        assert(rnum < 65)
        rnum - 1
    }
    %io => 0x40`64
}

#subruledef JumpInstructions {
    jmp => 0b0`32
    je => 0b1`32
    jne => 0b10`32
    jz => 0b11`32
    jg => 0b100`32
    jge => 0b101`32
    jl => 0b110`32
    jle => 0b111`32
}

#ruledef {
    nop => 0b0`32 @ 0xFFFFFFFF @ 0b0`64 @ 0b0`64
    hlt => 0b0`64 @ 0b0`64 @ 0b0`64

    mov {reg: Registers}, {value: Registers} => 0b1`32 @ 0b1`32 @ reg`64 @ value`64
    mov {reg: Registers}, {value: i64} => 0b10`32 @ 0b1`32 @ reg`64 @ value`64

    {opcode: JumpInstructions} {address: i64} => {
        reladdr = (address - $) / 8
        0b11`32 @ opcode @ reladdr`64 @ 0b0`64
    }
}