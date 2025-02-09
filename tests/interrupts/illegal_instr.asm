mov %it, interrupt_table
mov %sp, stack
.dw 0xffff ; fake an invalid instruction
.dw 0xffff 
hlt

interrupt_table:
  .dw illegal_instr_handler

illegal_instr_handler:
  mov %r1, 1
  hlt ;assert r1=1

.skip 16 ; some stack
stack:
