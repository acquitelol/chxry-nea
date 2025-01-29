; add %r3, %r2, %r1
; add %r3, %r2, 0x5
; sub %r3, %r2, 0b100
; test:
; lb %r4, %r2
; lbu %r2, 0x5
; lw %r6, %r3, 2
; jeq %r2
; jne test
; jlt %r4, 9
; nop
; mov %r2, 5
; mov %r1, %r5
; neg %r2, %r8
; not %r2, %r3
; cmp %r2, %r8
; cmp %r2, 44
; jmp 5
; jmp %r2
; jmp %r2, 5
test:
add %r1, %r1, 1
add %r2, %r2, %r1
jmp test
