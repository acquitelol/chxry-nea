start:
  ; mov %r1, 732
  ; loop:
  ;   rem %r2, %r1, 10
  ;   add %r2, %r2, 48
  ;   sb %r2, 0xf002
  ;   div %r1, %r1, 10
  ;   cmp %r1, 0
  ;   jgt loop
  ; rev_loop:
    
  hlt

itoa:
  mov %r7, 0
  itoa_loop:
    rem %r8, %r1, 10
    add %r8, %r8, 48
    sb %r2, %r3, 
    div %r1, %r1, 10
    cmp %r1, 0
    jgt itoa_loop
