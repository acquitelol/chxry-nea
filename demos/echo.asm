start:
  lw %r1, 0xf000
  cmp %r1, 0
  jeq start
  lbu %r1, 0xf002
  sb %r1, 0xf002
  
  jmp start
