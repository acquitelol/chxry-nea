sub %r1, %r0, 3
sub %r2, %r1, 3
sub %r3, %r2, %r1
hlt ;assert r1=65533 (-3), r2=65530 (-6), r3=65533 (-3)

neg %r4, %r3
neg %r5, %r4
hlt ;assert r4=3, r5=65533 (-3)
