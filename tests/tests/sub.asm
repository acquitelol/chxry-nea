sub %r1, %r0, 3
sub %r2, %r1, 3
sub %r3, %r2, %r1
hlt ;assert r1=65533 r2=65530 r3=65533
neg %r4, %r3
hlt ;assert r4=3
