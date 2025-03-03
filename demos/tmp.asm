start:
  mov %r1, 0

  mov %r1, 20
  mov %r2, 25
  mov %r3, 20
  mov %r4, 25
  mov %r5, 0b11000111
  add %ra, %pc, 4
  jmp draw_rect
  
  hlt
  jmp start

; x: %r1-%r2, y: %r3-%r4, %r5, clobbers %r6, %r7
draw_rect:
  mov %r7, %r1
  draw_rect_y:
    draw_rect_x:
      mul %r6, %r3, 128
      add %r6, %r6, %r1
      sb %r5, %r6, 0xc000

      inc %r1
      cmp %r1, %r2
      jgt draw_rect_x

    mov %r1, %r7
    inc %r3
    cmp %r3, %r4
    jgt draw_rect_y
  jmp %ra
