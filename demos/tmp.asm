start:
  mov %r8, 0xee
  loop:
  lb %r1, 0xf000
  lb %r2, 0xf001

  mul %r2, %r2, 128
  add %r2, %r2, %r1
  sb %r8, %r2, 0xc000
  cmp %r2, %r3
  jeq loop
  sb %r0, %r3, 0xc000
  mov %r3, %r2
  
  jmp loop
