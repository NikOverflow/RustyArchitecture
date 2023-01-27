#subruledef Registers {
    %reg{rnum: u7} => {
        assert(rnum > 0)
        assert(rnum < 65)
        rnum - 1
    }
    %io => 0x40`64
}

;#subruledef ComparisonJumpInstructions {
    ;je => 0x0003000000000001
    ;jne => 0x0003000000000002
    ;jz => 0x0003000000000003
    ;jg => 0x0003000000000004
    ;jge => 0x0003000000000005
    ;jl => 0x0003000000000006
    ;jle => 0x0003000000000007
;}

#ruledef {
    nop => 0b0`32 @ 0xFFFFFFFF @ 0b0`64 @ 0b0`64
    hlt => 0b0`64 @ 0b0`64 @ 0b0`64

    mov {reg: Registers}, {value: Registers} => 0b1`32 @ 0b1`32 @ reg`64 @ value`64
    mov {reg: Registers}, {value: i64} => 0b10`32 @ 0b1`32 @ reg`64 @ value`64




    ;nop => 0b0`64 @ 0b0`64 @ 0b0`64

    ;mov {reg: Registers}, {value: Registers} => 0b1`64 @ reg`64 @ value`64
    ;mov {reg: Registers}, {value: i64} => 0b10000001`64 @ reg`64 @ value`64



    ;add {reg: Registers}, {value: Registers} => 0b00010000`64 @ reg`64 @ value`64
    ;add {reg: Registers}, {value: i64} => 0b10010000`64 @ reg`64 @ value`64

    ;sub {reg: Registers}, {value: Registers} => 0b00010001 @ reg`64 @ value`64
    ;sub {reg: Registers}, {value: i64} => 0b10010001 @ reg`64 @ value`64 ; TODO: add reverse subtraction.

    ;imul {reg: Registers}, {value: Registers} => 0b00010010 @ reg`64 @ value`64
    ;imul {reg: Registers}, {value: i64} => 0b10010010 @ reg`64 @ value`64

    ;idiv {reg: Registers}, {value: Registers} => 0b00010011 @ reg`64 @ value`64
    ;idiv {reg: Registers}, {value: i64} => 0b10010011 @ reg`64 @ value`64 ; TODO: add reverse division.

    ;inc {reg: Registers} => asm {
        ;add {reg}, 1
    ;}

    ;dec {reg: Registers} => asm {
        ;sub {reg}, 1
    ;}

    ;zero {reg: Registers} => asm {
        ;mov {reg}, 0
    ;}



    ;and {reg: Registers}, {value: Registers} => 0x0002000000000000 @ reg`64 @ value`64
    ;and {reg: Registers}, {value: i64} => 0x1002000000000000 @ reg`64 @ value`64

    ;or {reg: Registers}, {value: Registers} => 0x0002000000000001 @ reg`64 @ value`64
    ;or {reg: Registers}, {value: i64} => 0x1002000000000001 @ reg`64 @ value`64

    ;xor {reg: Registers}, {value: Registers} => 0x0002000000000002 @ reg`64 @ value`64
    ;xor {reg: Registers}, {value: i64} => 0x1002000000000002 @ reg`64 @ value`64

    ;not {reg: Registers} => 0x0002000000000003 @ reg`64 @ 0x0`64

    ;neg {reg: Registers} => 0x0002000000000004 @ reg`64 @ 0x0`64

    ;nand {reg: Registers}, {value: Registers} => asm {
        ;and {reg}, {value}
        ;not {reg}
    ;}
    ;nand {reg: Registers}, {value: i64} => asm {
        ;and {reg}, {value}
        ;not {reg}
    ;}

    ;nor {reg: Registers}, {value: Registers} => asm {
        ;or {reg}, {value}
        ;not {reg}
    ;}
    ;nor {reg: Registers}, {value: i64} => asm {
        ;or {reg}, {value}
        ;not {reg}
    ;}

    ;xnor {reg: Registers}, {value: Registers} => asm {
        ;xor {reg}, {value}
        ;not {reg}
    ;}
    ;xnor {reg: Registers}, {value: i64} => asm {
        ;xor {reg}, {value}
        ;not {reg}
    ;}

    ;hlt => 0b01111111`64 @ 0b0`64 @ 0b0`64
}