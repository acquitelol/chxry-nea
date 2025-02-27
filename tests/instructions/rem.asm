mov %r8, 12
mov %r7, 65524 ; -12

rem %r1, %r8, 5
rem %r2, %r8, 6
rem %r3, %r8, 65531 ; -5
rem %r4, %r8, 0
rem %r5, %r8, %r3
rem %r6, %r7, 5

hlt ;assert r1=2, r2=0, r3=2, r4=65535 (0xffff), r5=0, r6=65534 (-2)
