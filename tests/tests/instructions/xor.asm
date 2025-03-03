mov %r8, 0b1100
mov %r7, 0b1010

xor %r1, %r8, %r7
xor %r2, %r8, 0b1111
xor %r3, %r8, %r0

hlt ;assert r1=6 (0b0110), r2=3 (0b0011), r3=12 (0b1100)
