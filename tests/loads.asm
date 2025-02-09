lb %r1, test_unsigned
lb %r2, test_signed
lbu %r3, test_unsigned
lbu %r4, test_signed
lw %r5, test_word
hlt ;assert r1=5, r2=65531 (-5), r3=5, r4=251, r5=64507 (-1029)

test_unsigned:
.db 5

test_signed:
.db 0xfb ; 251

test_word:
.dw 0xfbfb ; -1029

; todo test offsets
