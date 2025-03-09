outer_loop:
  mov %r1, 0
  mov %r2, 0
  inner_loop:
    add %r3, %r1, 2
    lw %r7, %r1, list
    lw %r8, %r3, list
    cmp %r7, %r8
    jgt no_swap

    sw %r8, %r1, list
    sw %r7, %r3, list
    mov %r2, 1
    no_swap:

    add %r1, %r1, 2
    cmp %r1, 18 ; (n-1)*2
    jne inner_loop
  cmp %r2, 0
  jne outer_loop

mov %r1, 0
assert_loop:
  lw %r2, %r1, list
  hlt
  add %r1, %r1, 2
  jmp assert_loop

list:
  .dw 37    ;assert r2=0
  .dw 13    ;assert r2=13 
  .dw 138   ;assert r2=20 
  .dw 498   ;assert r2=37
  .dw 20    ;assert r2=45 
  .dw 0     ;assert r2=138 
  .dw 382   ;assert r2=293 
  .dw 20391 ;assert r2=382 
  .dw 293   ;assert r2=498 
  .dw 45    ;assert r2=20391
