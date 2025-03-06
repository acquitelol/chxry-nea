mov %sp, stack

mov %r1, 7
add %ra, %pc, 4
jmp factorial

hlt ;assert r1=5040

; naive factorial implementation to demonstrate function calls and recursion using the stack
; r1<-r1!, clobbers r2
factorial:
  cmp %r1, 0
  jne factorial_else
  mov %r1, 1
  jmp %ra

  factorial_else:
    sub %sp, %sp, 4
    sw %r1, %sp, 2
    sw %ra, %sp
  
    sub %r1, %r1, 1
    add %ra, %pc, 4
    jmp factorial
  
    lw %ra, %sp
    lw %r2, %sp, 2
    add %sp, %sp, 4

  mul %r1, %r1, %r2
  jmp %ra

.skip 512 ; allocate stack
stack:
