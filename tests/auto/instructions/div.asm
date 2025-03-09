mov %r8, 12
mov %r7, 65524 ; -12

div %r1, %r8, 4
div %r2, %r8, 65532 ; -4
div %r3, %r8, 0
div %r4, %r8, %r1
div %r5, %r7, 4
div %r6, %r7, %r2

hlt ;assert r1=3, r2=65533 (-3), r3=65535 (0xffff), r4=4, r5=65533 (-3), r6=4
