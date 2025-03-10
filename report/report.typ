#import "@preview/zebraw:0.4.6": *
#let details = toml("conf.toml")

#set text(hyphenate: false, 12pt)
#set heading(numbering: "1.")
#set enum(numbering: "1.", full: true)
#show link: underline
#show ref: it => {
  let el = it.element
  if el != none {
    link(el.location(), [#numbering(el.numbering, ..counter(el.func()).at(el.location())) #el.body])
  }
}

#page(numbering: none, [
  #v(2fr)
  #align(center, [
    #text(20pt, weight: 700, [Assembler and emulator for a custom CPU architecture])
    #v(0.1fr)
    #text(16pt, [A-Level Computer Science NEA])
  ])
  #v(2fr)
  #align(right, [
    #text(16pt, details.name) \
    #details.college
  ])
])

#set par(justify: true)
#set page(margin: 2cm, footer: [*Centre Number:* #details.center_num #h(1fr) #context counter(page).display("1") #h(1fr) *Candidate Number:* #details.candidate_num])

#page(outline(indent: true))

= Analysis

== Project Outline

I plan to create a set of tools for programming with a custom instruction set, including an assembler and an interactive virtual machine. The goal is to make learning low level programming more approachable, by eliminating the complexity of modern computer architectures. The design and tooling for this instruction set will be based on real-life designs in order to make the skills learnt by the user transferrable to real technologies.
// TODO one more para probably? demo programs

== Client Information

The client is Gosha Tnimov, a student of computer science interested in low-level programming. He is looking for a way to learn assembly programming without the daunting task of using architectures like those from the Arm or x86 families. For this I designed a simple RISC instruction set, designed to be similar to real instruction sets. After conducting an interview, we settled on a list of features for the assembler and emulator suite:
- The assembler should have a CLI that functions similar to existing architectures.
- In the emulator you should be able to step forward by individual instructions, or run automatically at speeds up to 1 MHz.
- There should be a way to inspect and edit the register and memory state.
- There should be a way to save the emulator state to a file.
- The emulator should have a virtual display to write graphical programs.
- There should be a way to send keyboard input to the CPU.
- The toolchain should be shipped with a package of sample programs.
When asked about user interface, the client wanted the UI to be modular, with a way to arrange windows containing different information.

== Initial Research

=== Technologies

For all of the components I have chosen to use the Rust programming language for its high performance. This was especially a consideration for the emulator, as the virtual machine needs to be able to run at the clients target speed of at least 1 MHz, meaning each cycle has to be executed in under 1 #(sym.mu)s.

For the user interface, the client opted for a cross-platform desktop application, leaving different choices of how to implement the user interface.
- Dear ImGui #cite(<imgui>)
  - Implemented in C++, however there are Rust bindings availible.
  - Immediate mode UI, meaning each frame the UI is reconstructed based on the current state. This means no state has to be synchronised between UI elements and the backing data.
  - Widely used and documented.
  - Would require implementing a backend to draw the outputted vertex data; I could reuse the rendering backend from my EPQ for this.
- egui #cite(<egui>)
  - Implemented in Rust, meaning the API will be more ergonomic to use.
  - Immediate mode with a similar API to ImGui.
  - Also includes eframe, a cross platform backend.
- Tauri #cite(<tauri>)
  - A framework for building desktop applications using web technologies, similar to electron.
  - Using css to design the UI allows more flexibility than immediate mode libraries.
  - Data has to be constantly serialized and sent between the UI Javascript and the internal rust emulation code.
Based on these factors, I opted to use egui for the user interface.

=== Existing Architectures

// TODO citations here
- x86
  - A family of very complex architectures used in most desktop PC's.
  - Based on the Intel 8086 microprocessor released in 1978, and has evolved ever since.
  - Has different operating modes to be able to use 16-bit, 32-bit and 64-bit word sizes.
  - Contains many extensions to enable extra functionality like SIMD and floating point support.
  - Instruction-Memory architecture, meaning operations can be performed on memory locations as well as registers.
- Arm
  - A family of RISC architectures mostly used in mobile phones and laptops.
  - Contains Thumb, a subset of instructions used for embedded systems.
  - Load-Store architecture, meaning arithmetic operations can only occur between registers, data from memory must be loaded into a register first.
- RISC-V
  - An open source RISC instruct set architecture.
  - Contains different base instruction sets for 32-bit, 64-bit and 128-bit word sizes, along with extensions for feautres like multiplication and floating point.
  - Seperated into unprivileged instructions for most applications, and privileged for features like virtual memory meant to be used by operating systems and similar.

=== Similar Implementations

- ASTRO-8 #cite(<astro8>)
  - An emulator and assembler for a 16-bit computer design.
  - Supports many different IO methods, including a virtual display, keyboard and mouse input and sound output.
  - Only has 3 general purpose registers, however supports multiple memory banks.
  - The emulator is a desktop app with a seperate assembler program.
  - The emulator only shows the display output, and provides no debugging information.
- yasp #cite(<yasp>)
  - A web based assembler development environment.
  - Simulates different hardware devices (LEDs, buttons etc.).
  - The code writing experience is interactive, with live error checking and helpful information when hovering instructions.
  - Has a big focus on debugging, with breakpoints and the ability to step forward and backward through instructions.
  - Can only run at \~25 KHz.
  - #figure(caption: [The yasp user interface, showing the assembly code next to the debugger output.], image("yasp.png"))

== Objectives

+ _Assembler_
  + The assembler should be able to load an assembly file from the command line arguments.
    + If the given filename doesn't exist, or isn't valid UTF-8 the assembler should display an error and exit.
  + If no command line arguments are given, the assembler should show a help message.
  + The assembler should iterate through all of the lines of the program.
    + It should skip comments (lines beginning with a semicolon) and lines consisting only of whitespace.
    + It should parse each opcode mnemonic, then a comma separated list of operands.
    + If the mnemonic is unrecognized the program should display an error and exit.
    + It should treat the opcode and operands as case-insensitive, unless the operand is a string literal.
    + It should be able to parse numeric literals of different bases (0xA3, 0b0111...).
    + If the given base for a numeric literal is invalid it should display an error and exit.
    + If an operand begins with a `%`, it should be treated as a register.
    + If an invalid register name is used, it should display and error and exit.
    + It should verify if the given operands are compatible with the opcode, otherwise it should display an error and exit.
    + If a label is defined (string followed by colon), it should insert the current position and label name into a symbol map.
    + If a label is used, it should insert the current position and label name into a symbol usages map.
    + There should be pseudo instructions to define raw data.
    + The assembler should emit the machine code for each instruction to a binary buffer.
  + The assembler should output a binary object file, containing the symbol maps and machine code.
+ _Linker_
  + The linker should load all the object files from the command line arguments.
    + If any of the filenames don't exist, or could not be parsed, then it should display an error and exit.
  + The linker should parse each given object file.
    + It should verify that each file begins with the correct header, otherwise it should display an error and exit.
    + It should merge the symbol maps of all files together.
    + If a symbol is defined multiple times, it should display an error and exit.
    + The machine code should be appended to a buffer.
  + The linker should iterate through the symbol usages
    + If a symbol isn't defined in any of the symbol maps, it should show an error and exit.
    + It should insert the location of the symbol definition into the machine code at the usage address.
  + The linker should output the final machine code to a binary file given in the command line arguments.
+ _Emulator_
  + The emulator should be able to load a machine code file into memory.
  + The emulator should be able to save its registers and memory to a file.
  + The emulator should be able to load its registers and memory from a file.
  + The state of the CPU and memory should be able to be saved/loaded from a file.
  + The emulator should execute execute the instruction at the virtual program counter if it is unpaused or the user requested to step 1 instruction.
    + It should decode the instruction at the program counter.
    + If the decoded opcode is invalid, #strike([an interrupt should be raised]) the register state should be reset.
    + The decoded instruction should be executed, and the registers and memory should be updated accordingly.
  + The emulator should have a user interface with multiple panels inside.
    + These panels should be able to be moved, resized, opened and closed by the user.
  + There should be a window to control the current CPU state.
    + There should be a button to pause/resume the CPU execution based on current state.
    + There should be a button to step the CPU forward one cycle.
    + There should be a way to vary the execution speed of the CPU.
    + This window should display if the CPU is currently active or not.
    + This window should show the the decoded string of the last instruction.
  + There should be a window to display the CPU registers.
    + Whilst the emulation is paused, the register values should be editable by the user.
    + It should verify whether the inputted value is valid for the base the number is in.
  + There should be a window to inspect the memory.
    + This window should display the memory address for each row.
    + The memory data should be displayed in hexadecimal.
    + Whilst the emulation is paused, the user should be able to edit individual bytes.
    + It should verify whether the input is valid hexadecimal.
  + There should be a window to view a virtual display.
    + The data for the display should be mapped to a region in the emulated memory.
    + #strike([With this window selected, any keyboard inputs should be sent to the CPU as interrupts.])
  + There should be a window to interact with a virtual serial port.
    + The user should be able to type in an a message, which will be then encoded as UTF-8 and queued to be sent to the CPU.
    + If the emulated program reads from a specific address, the value read should be popped from the queue.
    + If the emulated program writes to a specific address, it should be decoded as UTF-8 and displayed in the window.

= Documented Design

== Project Structure

This project will contain 5 rust crates,
- q16: The library where most of the logic is implemented. This is so the emulation and assembly logic can be reused between the emulator and tests. This library also hosts the enums that define the values assigned for each opcode and register.
- q16-asm: The assembler CLI.
- q16-ld: The linker CLI. Used to link together multiple object files produced by the assembler.
- q16-emu: The emulator. A graphical application that can load machine code that has been linked and run programs interactively.
- q16-tests: An automated test runner that assembles and runs programs and compares the registers to expected outputs.

== Key Structures

=== `Assembler`

=== `Operand`

=== `Obj`

=== `Emulator`

=== `Registers`

=== `Instruction`

=== `CircularBuffer`

=== `ArgParser`

=== `EmuState`

=== `App`

== File Formats

=== Object File

=== Emulator State File

== UI Design

== Instruction set

q16 is a 16-bit, little-endian RISC instruction set designed for this project. It is inspired heavily by RISC-V. This section exists as a reference for implementation, and a guide for the end user.

=== Registers
There are 13 programmer available registers, all of which are 16-bit. Registers are referenced in instructions using 4-bit identifiers.
#figure(
  table(
    columns: 3,
    [*Id*], [*Name*], [*Description*],
    [`0000`], [`r0`], [Hardwired to zero. Writes are a no-op.],
    [`0001-1000`], [`r1-r8`], [General purpose registers.],
    [`1001`], [`pc`], [Program counter.],
    [`1010`], [`sp`], [Stack pointer.],
    [`1011`], [`ra`], [Return address.],
    [`1100`], [`sts`], [Status register.]
  ),
  caption: [List of registers]
)
The contents of the status register are defined as follows.
#figure(
  table(
    columns: 3,
    [*Bit*], [*Name*], [*Description*],
    [`0`], [Zero], [If last ALU result was 0],
    [`1`], [Negative], [If last ALU result was negative],
    [`2:7`], table.cell(colspan: 2, [Reserved]),
    [`8`], [Run], [If the CPU is running.],
    [`9:15`], table.cell(colspan: 2, [Reserved])
  ),
  caption: [Status register bitfield]
)

=== Instruction Format
All instructions are 32 bits long, with the first 8 bits representing the opcode. The most significant bit of the opcode denotes the instruction format. The base instruction formats are shown with the least significant bit first.

#let instrformat(cells) = grid(
  columns: range(32).map(_ => 1fr),
  rows: (2.5pt, auto),
  stroke: 1pt,
  inset: 5pt,
  align: center,
  ..range(32).map(_ => grid.cell(stroke: (top: none, right: none), [])),
  ..cells.map(c => grid.cell(colspan: c.size, raw(c.label)))
)

*Type R:*
#instrformat((
  (size: 7, label: "opcode 0:6"),
  (size: 1, label: "0"),
  (size: 4, label: "rd 8:11"),
  (size: 4, label: "r1 12:15"),
  (size: 4, label: "r2 16:19"),
  (size: 12, label: "unused"),
))

*Type I:*
#instrformat((
  (size: 7, label: "opcode 0:6"),
  (size: 1, label: "1"),
  (size: 4, label: "rd 8:11"),
  (size: 4, label: "r1 12:15"),
  (size: 16, label: "imm 16:31"),
))

(`rd` - destination register. `r1/r2` - source registers. `imm` - 16-bit immediate value.)

=== Opcodes

#let ritype(opcode, mnemonic, s) = (
    raw("0" + opcode), [`R`], raw(mnemonic), [`rd` #sym.arrow.l `r1` #s `r2`],
    raw("1" + opcode), [`I`], raw(mnemonic), [`rd` #sym.arrow.l `r1` #s `imm`],
)

#figure(
  table(
    columns: 4,
    [*Opcode*], [*Format*], [*Mnemonic*], [*Description*],
    table.cell(colspan: 4, [_Arithmetic/Logic operations_]),
    ..ritype("0000001", "add", [`+`]),
    [`00000010`], [`R`], [`sub`], [`rd` #sym.arrow.l `r1` `-` `r2`],
    ..ritype("0000011", "mul", sym.times),
    ..ritype("0000100", "div", sym.div),
    ..ritype("0000101", "rem", [`%`]),
    ..ritype("0000110", "and", sym.and),
    ..ritype("0000111", "or", sym.or), 
    ..ritype("0001000", "xor", sym.xor),
    table.cell(colspan: 4, [_Memory operations_]),
    [`10001001`], [`I`], [`lb`], [Loads 8 bits from `[r1+imm]` into `rd` sign extended.],
    [`10001010`], [`I`], [`lbu`], [Loads 8 bits from `[r1+imm]` into `rd` zero extended.],
    [`10001011`], [`I`], [`lw`], [Loads 16 bits from `[r1+imm]` into `rd`.],
    [`10001100`], [`I`], [`sb`], [Writes 8 bits from `rd` to `[r1+imm]`.],
    [`10001101`], [`I`], [`sw`], [Writes 16 bits from `rd` to `[r1+imm]`.],
    table.cell(colspan: 4, [_Branching operations_]),
    [`10001110`], [`I`], [`jeq`], [Jump to `r1+imm` if zero flag set],
    [`10001111`], [`I`], [`jne`], [Jump to `r1+imm` if zero flag not set],
    [`10010000`], [`I`], [`jgt`], [Jump to `r1+imm` if negative flag set],
    [`10010001`], [`I`], [`jlt`], [Jump to `r1+imm` if negative flat not set],
  ),
  caption: [List of opcodes]
)
The branching instructions use the value in the `sts` register, and are designed to be used with the `cmp` pseudo-instruction, although this is not necessarily required. For the memory and branching operations, either one of `r1` or `imm` can be omitted.

// move to assembly section along with instruction format usages
=== Pseudo Instructions
Many assembly instructions are implemented using other instructions. 

#figure(
  table(
    columns: 2,
    [*Usage*], [*Implementation*],
    [`nop`], [`add %r0, %r0, %r0`],
    [`hlt`], [`and %sts, %sts, 0xeff`],
    [`mov %rd, %r1/imm`], [`add %rd, %r0, %r1/imm`],
    [`sub %r1, imm`], [`add %r0, %r1, -imm`],
    [`neg %rd, %r1`], [`sub %rd, %r0, %r1`],
    [`not %rd, %r1`], [`xor %rd, %r1, -1`],
    [`cmp %r1, %r2`], [`sub %r0, %r1, %r2`],
    [`cmp %r1, imm`], [`add %r0, %r1, -imm`],
    [`jmp %r1/imm`], [`mov %pc, %r1/imm`],
    [`jmp %r1, imm`], [`add %pc, %r1, imm`],
    [`inc %rd`], [`add %rd, %rd, 1`],
  ),
  caption: [List of pseudo-instructions]
)

= Technical Solution

== Skills Demonstrated

// TODO

== Source Code

#let sourcecode(lang: "rust", path) = [
  #set par(justify: false)
  === #raw(path)
  #zebraw(lang: false, text(10pt, raw(lang: lang, block: true, read("../" + path))))
]

// todo have descriptions for the files and crates
#sourcecode("q16/src/lib.rs")
#sourcecode("q16/src/asm.rs")
#sourcecode("q16/src/obj.rs")
#sourcecode("q16/src/emu.rs")
#sourcecode("q16/src/util.rs")
#sourcecode("asm/src/main.rs")
#sourcecode("ld/src/main.rs")
#sourcecode("emu/src/main.rs")
#sourcecode("emu/src/ui/mod.rs")
#sourcecode("emu/src/ui/cpu_state.rs")
#sourcecode("emu/src/ui/memory.rs")
#sourcecode("emu/src/ui/display.rs")
#sourcecode("emu/src/ui/serial.rs")
#sourcecode("emu/src/ui/log.rs")
#sourcecode("tests/src/main.rs")

#sourcecode(lang: "asm", "demos/base.asm")
#sourcecode(lang: "asm", "demos/mandelbrot.asm")
#sourcecode(lang: "asm", "demos/gameoflife.asm")
#sourcecode(lang: "asm", "demos/fibonacci.asm")
#sourcecode(lang: "asm", "demos/echo.asm")

= Testing

// TODO WRITE STATUS REGISTER TEST

= Evaluation

= Bibliography

#bibliography(title: none, full: true, "bibliography.yml")
