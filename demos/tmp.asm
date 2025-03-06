start:
  mov %r1, 1387
  add %ra, %pc, 4
  jmp print_int
    
  hlt
  jmp start

print_int:
  mov %r2, %sp
  conv_loop:
    rem %r3, %r1, 10
    add %r3, %r3, 48
    div %r1, %r1, 10
    
    sub %sp, %sp, 1
    sb %r3, %sp

    cmp %r1, 0
    jne conv_loop

  output_loop:
    lb %r3, %sp
    sb %r3, 0xf002
    add %sp, %sp, 1
  
    cmp %r2, %sp
    jne output_loop

  jmp %ra
