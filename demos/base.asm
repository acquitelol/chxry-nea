mov %sp, start
mov %it, interrupt_table

jmp start

stack:
.skip 1024

interrupt_table:
  .dw illegal_instr_handler

illegal_instr_handler:
  mov %r7, 7
  hlt
