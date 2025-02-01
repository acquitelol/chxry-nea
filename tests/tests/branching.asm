mov %r3, 3

; test jmp
jmp jmp1 ; should jump
hlt
jmp1:
inc %r8
hlt
;assert r8=1

; test jeq
cmp %r3, 3
jeq jeq1 ; should jump
hlt
jeq1:
inc %r8
hlt
;assert r8=2

cmp %r3, 2
jeq die ; should not jump
inc %r8
hlt
;assert r8=3

; test jne
cmp %r3, 2
jne jne1 ; should jump
hlt
jne1:
inc %r8
hlt
;assert r8=4

cmp %r3, 3
jne die ; should not jump
inc %r8
hlt
;assert r8=5

; test jgt
cmp %r3, 4
jgt jgt1 ; should jump
hlt
jgt1:
inc %r8
hlt
;assert r8=6

cmp %r3, 2
jgt die ; should not jump
inc %r8
hlt
;assert r8=7

cmp %r3, 3
jgt die ; should not jump
inc %r8
hlt
;assert r8=8

; test jlt
cmp %r3, 2
jlt jlt1 ; should jump
hlt
jlt1:
inc %r8
hlt
;assert r8=9

cmp %r3, 4
jlt die ; should not jump
inc %r8
hlt
;assert r8=10

cmp %r3, 3
jlt die ; should not jump
inc %r8
hlt
;assert r8=11

die:
hlt
