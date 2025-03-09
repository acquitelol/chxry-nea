mov %r8, 99
sb %r8, test_byte
sw %r8, test_word

lbu %r1, test_byte
lw %r2, test_word
hlt ;assert r1=99 r2=99

test_byte:
.db 5

test_word:
.dw 15
