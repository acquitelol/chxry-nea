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
    add %r4, %r1, 0xbf7f
    add %ra, %pc, 4
    jmp wrap_r4
    lbu %r3, %r4, 0 ; up left
    add %r2, %r0, %r3

    add %r4, %r1, 0xbf80
    add %ra, %pc, 4
    jmp wrap_r4
    lbu %r3, %r4, 0 ; up
    add %r2, %r2, %r3

    add %r4, %r1, 0xbf81
    add %ra, %pc, 4
    jmp wrap_r4
    lbu %r3, %r4, 0 ; up right
    add %r2, %r2, %r3

    add %r4, %r1, 0xbfff
    add %ra, %pc, 4
    jmp wrap_r4
    lbu %r3, %r4, 0 ; left
    add %r2, %r2, %r3

    add %r4, %r1, 0xc001
    add %ra, %pc, 4
    jmp wrap_r4
    lbu %r3, %r4, 0  ; right
    add %r2, %r2, %r3

    add %r4, %r1, 0xc07f
    add %ra, %pc, 4
    jmp wrap_r4
    lbu %r3, %r4, 0 ; down left
    add %r2, %r2, %r3

    add %r4, %r1, 0xc080
    add %ra, %pc, 4
    jmp wrap_r4
    lbu %r3, %r4, 0 ; down
    add %r2, %r2, %r3

    add %r4, %r1, 0xc081
    add %ra, %pc, 4
    jmp wrap_r4
    lbu %r3, %r4, 0 ; down right
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

wrap_r4:
    cmp %r4, 0xc000
    jge out1
    add %r4, %r4, 0x3000
  out1:
    cmp %r4, 0xefff
    jle out2
    sub %r4, %r4, 0x3000
  out2:
    jmp %ra
