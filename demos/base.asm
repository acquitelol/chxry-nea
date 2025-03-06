mov %sp, stack

jmp start

stack: ; allocate stack
.skip 1024
