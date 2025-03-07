mov %sp, stack

jmp start

.skip 1024
stack: ; allocate stack
