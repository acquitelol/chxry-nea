add %r1, %r0, 3
add %r2, %r1, 3
add %r3, %r2, %r1
hlt ;assert r1=3 r2=6 r3=9
inc %r3
inc %r4
hlt ;assert r3=10 r4=1
