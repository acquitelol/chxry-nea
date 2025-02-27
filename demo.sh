#!/bin/bash
set -xe
cargo run --bin q16-asm -- demos/base.asm
cargo run --bin q16-asm -- demos/$1.asm
cargo run --bin q16-ld -- $1.bin demos/base.o demos/$1.o
cargo run --bin q16-emu --release -- $1.bin

