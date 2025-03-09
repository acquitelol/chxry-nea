PATH := $(PATH):target/release

run-tests: q16-tests
	q16-tests
	
run-%: demos/%.bin q16-emu
	q16-emu -b $<

%.bin: %.o demos/base.o q16-ld
	q16-ld demos/base.o $< -o $@

%.o: %.asm q16-asm
	q16-asm $< -o $@

watch-docs:
	typst watch --root . report/report.typ

q16-%:
	cargo build --release --bin $@
	
