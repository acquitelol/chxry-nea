start:
  add %ra, %pc, 4
  jmp mandelbrot
   
  hlt
  jmp start

; draw the mandelbrot set centered at (-0.75, 0), clobbers r1-8
mandelbrot:
  mov %r1, 0
  mandelbrot_display_loop:
    ; cx=r2, cy=r3, x=r4, y=r5, iter=r6
    rem %r2, %r1, 128
    mul %r2, %r2, 2
    sub %r2, %r2, 192 ; cx = px * 2 - 192
    div %r3, %r1, 128
    mul %r3, %r3, 2
    sub %r3, %r3, 96 ; cy = py * 2 - 96
    mov %r4, 0
    mov %r5, 0
    mov %r6, 0
    mandelbrot_iter_loop:
      mul %r7, %r4, %r4 ; r7 = x^2
      mul %r8, %r5, %r5 ; r8 = y^2
      add %r8, %r7, %r8 ; r8 = x^2+y^2
      cmp %r8, 32400 ; break if x^2+y^2 > 4 * 90^2
      jlt mandelbrot_iter_continue
      mul %r8, %r5, %r5 ; r8 = y^2
      sub %r8, %r7, %r8 ; r8 = x^2-y^2
      div %r7, %r8, 90 ; r7 = (x^2-y^2)/90
      mul %r8, %r4, %r5 ; r8 = xy
      div %r8, %r8, 45 ; r8 = 2xy/90
      add %r4, %r7, %r2 ; x = (x^2-y^2)/90 + cx
      add %r5, %r8, %r3 ; y = 2xy/90 + cy

      inc %r6
      cmp %r6, 50 ; 50 iterations
      jgt mandelbrot_iter_loop
    jmp mandelbrot_draw_skip

    mandelbrot_iter_continue:
    div %r6, %r6, 10
    add %r6, %r6, 0b10001100
    sb %r6, %r1, 0xc000 ; draw pixel
    mandelbrot_draw_skip:
    
    inc %r1
    cmp %r1, 0x3000 ; 128*96
    jne mandelbrot_display_loop
  
  jmp %ra
