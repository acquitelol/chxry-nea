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
    cmp %r4, 0xc000
    jge out1
    add %r4, %r4, 0x3000
  out1:
    cmp %r4, 0xefff
    jle out2
    sub %r4, %r4, 0x3000
  out2:
    lbu %r3, %r4, 0 ; up left
    add %r2, %r0, %r3

    add %r4, %r1, 0xbf80
    cmp %r4, 0xc000
    jge out3
    add %r4, %r4, 0x3000
  out3:
    cmp %r4, 0xefff
    jle out4
    sub %r4, %r4, 0x3000
  out4:
    lbu %r3, %r4, 0 ; up
    add %r2, %r2, %r3

    add %r4, %r1, 0xbf81
    cmp %r4, 0xc000
    jge out5
    add %r4, %r4, 0x3000
  out5:
    cmp %r4, 0xefff
    jle out6
    sub %r4, %r4, 0x3000
  out6:
    lbu %r3, %r4, 0 ; up right
    add %r2, %r2, %r3

    add %r4, %r1, 0xbfff
    cmp %r4, 0xc000
    jge out7
    add %r4, %r4, 0x3000
  out7:
    cmp %r4, 0xefff
    jle out8
    sub %r4, %r4, 0x3000
  out8:
    lbu %r3, %r4, 0 ; left
    add %r2, %r2, %r3

    add %r4, %r1, 0xc001
    cmp %r4, 0xc000
    jge out9
    add %r4, %r4, 0x3000
  out9:
    cmp %r4, 0xefff
    jle out10
    sub %r4, %r4, 0x3000
  out10:
    lbu %r3, %r4, 0  ; right
    add %r2, %r2, %r3

    add %r4, %r1, 0xc07f
    cmp %r4, 0xc000
    jge out11
    add %r4, %r4, 0x3000
  out11:
    cmp %r4, 0xefff
    jle out12
    sub %r4, %r4, 0x3000
  out12:
    lbu %r3, %r4, 0 ; down left
    add %r2, %r2, %r3

    add %r4, %r1, 0xc080
    cmp %r4, 0xc000
    jge out13
    add %r4, %r4, 0x3000
  out13:
    cmp %r4, 0xefff
    jle out14
    sub %r4, %r4, 0x3000
  out14:
    lbu %r3, %r4, 0 ; down
    add %r2, %r2, %r3

    add %r4, %r1, 0xc081
    cmp %r4, 0xc000
    jge out15
    add %r4, %r4, 0x3000
  out15:
    cmp %r4, 0xefff
    jle out16
    sub %r4, %r4, 0x3000
  out16:
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
