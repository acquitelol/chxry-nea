start:
  mov %r8, 0xff
  sb %r8, 0xc285
  sb %r8, 0xc304
  sb %r8, 0xc305
  sb %r8, 0xc385
  sb %r8, 0xc386
  
  sb %r8, 0xca14
  sb %r8, 0xca95
  sb %r8, 0xcb15
  sb %r8, 0xcb14
  sb %r8, 0xcb13
  hlt
  
  cycle:
  mov %r1, 0
  game_loop:
    lbu %r3, %r1, 0xbf7f ; up left
    add %r2, %r0, %r3
    lbu %r3, %r1, 0xbf80 ; up
    add %r2, %r2, %r3
    lbu %r3, %r1, 0xbf81 ; up right
    add %r2, %r2, %r3
    lbu %r3, %r1, 0xbfff ; left
    add %r2, %r2, %r3
    lbu %r3, %r1, 0xc001 ; right
    add %r2, %r2, %r3
    lbu %r3, %r1, 0xc07f ; down left
    add %r2, %r2, %r3
    lbu %r3, %r1, 0xc080 ; down
    add %r2, %r2, %r3
    lbu %r3, %r1, 0xc081 ; down right
    add %r2, %r2, %r3
       
    lbu %r3, %r1, 0xc000
    cmp %r2, 0x1fe ; 2 * 0xff
    jeq continue
    cmp %r2, 0x2fd ; 3 * 0xff
    jne die
    mov %r3, 0xff
    jmp continue
    die:
    mov %r3, 0
    continue:
    sb %r3, %r1, 0x2000
    
    inc %r1
    cmp %r1, 0x3000
    jne game_loop
    
  mov %r1, 0
  blit_loop:
    lw %r2, %r1, 0x2000
    sw %r2, %r1, 0xc000
  
    add %r1, %r1, 2
    cmp %r1, 0x3000
    jne blit_loop
       
  jmp cycle
