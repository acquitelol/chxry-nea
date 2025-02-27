hlt ;assert r0=0 r1=0 r2=0 r3=0 r4=0 r5=0 r6=0 r7=0 r8=0 sp=0 ra=0 it=0 pc=4 
mov %r0, 999
mov %r1, 1
mov %r2, 2
mov %r3, 3
mov %r4, 4
mov %r5, 5
mov %r6, 6
mov %r7, 7
mov %r8, 8
mov %sp, 10
mov %ra, 11
mov %it, 12
hlt ;assert r0=0 r1=1 r2=2 r3=3 r4=4 r5=5 r6=6 r7=7 r8=8 sp=10 ra=11 it=12 pc=56 
