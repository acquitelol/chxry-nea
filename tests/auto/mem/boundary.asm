mov %r1, 99
sb %r1, 65535
lb %r2, 65535
hlt ;assert r2=99

mov %r1, 999
sw %r1, 65534
lw %r2, 65534
hlt ;assert r2=999

mov %r1, 999
sw %r1, 65535
lw %r2, 65535
hlt ;assert r2=231 (lower half of 999)
