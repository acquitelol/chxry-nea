mov %r8, 0b1100
mov %r7, 0b1010

and %r1, %r8, %r7
and %r2, %r8, 0b1111
and %r3, %r8, %r0

hlt ;assert r1=8 (0b1000), r2=12 (0b1100), r3=0
