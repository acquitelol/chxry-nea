mov %r8, 0b1100
mov %r7, 0b1010

or %r1, %r8, %r7
or %r2, %r8, 0b1111
or %r3, %r8, %r0

hlt ;assert r1=14 (0b1110), r2=15 (0b1111), r3=12 (0b1100)
