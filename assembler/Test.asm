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

;mov %reg1, 2
;add %reg1, 40
;hlt

; My First Program that should work:
;mov %reg1, 21
;mov %io, %reg1
;add %reg1, 21
;mov %io, %reg1



;nop
;mov %reg1, 42
;mov %reg1, %reg2
;hlt
mov %reg1, 0xDEADBEEF
nop
mov %reg2, %reg1
nop
mov %reg3, %reg2
nop
mov %reg1, %reg4
hlt