jmp start
.skip 1024 ; allocate stack

start:
  mov %sp, start
  mov %it, interrupt_table

  mov %r1, 2
  add %ra, %pc, 4
  jmp square

  add %ra, %pc, 4
  jmp factorial

  add %ra, %pc, 4
  jmp mandelbrot
   
  hlt
  jmp start

square:
  mul %r1, %r1, %r1
  jmp %ra

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

mandelbrot:
  mov %r1, 0
  mandelbrot_display_loop:
    ; cx=r2, cy=r3, x=r4, y=r5, iter=r6
    rem %r2, %r1, 128
    mul %r2, %r2, 5
    div %r2, %r2, 2
    sub %r2, %r2, 210 ; cx = px * 5/2 - 210
    div %r3, %r1, 128
    mul %r3, %r3, 5
    div %r3, %r3, 2
    sub %r3, %r3, 120 ; cy = py * 5/2 - 120
    mov %r4, 0
    mov %r5, 0
    mov %r6, 0
    mandelbrot_iter_loop:
      mul %r7, %r4, %r4 ; r7 = x^2
      mul %r8, %r5, %r5 ; r8 = y^2
      add %r8, %r7, %r8 ; r8 = x^2+y^2
      cmp %r8, 0x7fff ; ideally 40000 but thats out of i16 range
      jlt mandelbrot_iter_continue
      mul %r8, %r5, %r5 ; r8 = y^2
      sub %r8, %r7, %r8 ; r8 = x^2-y^2

      div %r7, %r8, 100 ; r7 = (x^2-y^2)/100
      mul %r8, %r4, %r5 ; r8 = xy
      div %r8, %r8, 50 ; r8 = 2xy/100
      add %r4, %r7, %r2 ; x = (x^2-y^2)/100 + cx
      add %r5, %r8, %r3 ; y = 2xy/100 + cy

      inc %r6
      cmp %r6, 50
      jgt mandelbrot_iter_loop

    mandelbrot_iter_continue:
    mul %r6, %r6, 5
    sb %r6, %r1, 0xc000 ; draw pixel

    inc %r1
    cmp %r1, 0x3000
    jne mandelbrot_display_loop
  
  jmp %ra

interrupt_table:
  .dw illegal_instr_handler

illegal_instr_handler:
  mov %r7, 7
  hlt
