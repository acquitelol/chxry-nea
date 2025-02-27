mov %r8, 3

mul %r1, %r8, 4
mul %r2, %r8, %r1
mul %r3, %r8, 65535 ; -1
mul %r4, %r3, %r3

hlt ;assert r1=12, r2=36, r3=65533 (-3), r4=9
