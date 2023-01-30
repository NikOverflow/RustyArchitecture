#include "RustyArchitecture.asm"

; mov <Destination Register>, <Integer/Register>

; add <Destination Register>, <Integer/Register>
; sub <Destination Register>, <Integer/Register>
; imul <Destination Register>, <Integer/Register>
; idiv <Destination Register>, <Integer/Register>
; inc <Destination Register>
; dec <Destination Register>

; and <Destination Register>, <Integer/Register>
; or <Destination Register>, <Integer/Register>
; xor <Destination Register>, <Integer/Register>
; not <Destination Register>
; neg <Destination Register>
; nand <Destination Register>, <Integer/Register>
; nor <Destination Register>, <Integer/Register>




;start:
    ;mov %reg1, 0xDEADBEEF
    ;jmp middle
;middle:
    ;mov %reg1, 0
    ;jmp start

mov %reg1, 0
start:
    add %reg1, 1
    cmp %reg1, 5
    je end
    jne start
end:
    hlt