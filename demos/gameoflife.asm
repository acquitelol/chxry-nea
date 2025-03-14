start:
  mov %r8, 0xff
  sb %r8, 0xd840 ; (64, 48)
  sb %r8, 0xd8c0 ; (64, 49)
  sb %r8, 0xd7c0 ; (64, 47)
  sb %r8, 0xd83f ; (63, 48)
  sb %r8, 0xd8c1 ; (65, 49)
  hlt

cycle:
  mov %r1, 0
  game_loop:
    ; this doesnt wrap around "correctly" at edges but is convincing enough
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

  ; the game array is first written to 0x2000 before being copied to vram
  mov %r1, 0
  blit_loop:
    lw %r2, %r1, 0x2000
    sw %r2, %r1, 0xc000

    add %r1, %r1, 2
    cmp %r1, 0x3000
    jne blit_loop
     
  jmp cycle
