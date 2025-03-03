mov %r1, 10
.dw 0xffff ; attempt an invalid instruction
.dw 0xffff
;assert pc=0 r1=0
