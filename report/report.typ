#import "@preview/zebraw:0.4.6": *
#let details = toml("conf.toml")

#set text(hyphenate: false, 12pt)
#set heading(numbering: "1.")
#set enum(numbering: "1.", full: true)
#show link: underline
#show ref: underline

#page(numbering: none, [
  #v(2fr)
  #align(center, [
    #text(22pt, weight: 700, [Interactive Emulator and Assembler for a RISC computer])
    #v(0.1fr)
    #text(16pt, [A-Level Computer Science NEA])
  ])
  #v(2fr)
  #align(right, [
    #text(16pt, details.name) \
    #details.college
  ])
])

// #set par(justify: true)
#set page(margin: 2cm, footer: [*Centre Number:* #details.center_num #h(1fr) #context counter(page).display("1") #h(1fr) *Candidate Number:* #details.candidate_num])

#page(outline(indent: true, depth: 3))

= Analysis

== Project Outline

I plan to create a set of tools for programming with a custom instruction set, including an assembler and an interactive virtual machine. The goal is to make learning low level programming more approachable, by eliminating the complexity of modern computer architectures. The design and tooling for this instruction set will be based on real-life designs in order to make the skills learnt by the user transferrable to real technologies.

An assembler is a compiler for programs written in assembly. This is the most primitive form of a programming language, where the instructions in the language correspond almost directly to the instructions in the architecture. Knowing how to write assembly programs may seem redundant with modern compilers, however many multimedia processing programs like FFmpeg and dav1d @ffmpeg use handwritten assembly for performance critical functions.

The assembler will compile one file at a time into object files, meaning a linker is required to combine multiple object files. The linker is also responsible for resolving labels, which are names that identify a location in memory. The linker will finally output a binary file that can be executed as machine code by the emulator.

For this project I also will write a small collection of programs to demonstrate the functionality of the assembly language and the emulator.

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
- Dear ImGui @imgui
  - Implemented in C++, however there are Rust bindings available.
  - Immediate mode UI, meaning each frame the UI is reconstructed based on the current state. This means no state has to be synchronised between UI elements and the backing data.
  - Widely used and documented.
  - Would require implementing a backend to draw the outputted vertex data; I could reuse the rendering backend from my EPQ for this.
- egui @egui
  - Implemented in Rust, meaning the API will be more ergonomic to use.
  - Immediate mode with a similar API to ImGui.
  - Also includes eframe, a cross-platform backend.
- Tauri @tauri
  - A framework for building desktop applications using web technologies, similar to electron.
  - Using css to design the UI allows more flexibility than immediate mode libraries.
  - Data must be constantly serialized and sent between the UI Javascript and the internal rust emulation code.
Based on these factors, I opted to use egui for the user interface.

=== Existing Architectures

// TODO citations here
- x86
  - A family of very complex architectures used in most desktop PC's.
  - Based on the Intel 8086 microprocessor released in 1978 and has evolved ever since.
  - Has different operating modes to be able to use 16-bit, 32-bit and 64-bit word sizes.
  - Contains many extensions to enable extra functionality like SIMD and floating point support.
  - Instruction-Memory architecture, meaning operations can be performed on memory locations as well as registers.
- Arm
  - A family of RISC architectures mostly used in mobile phones and laptops.
  - Contains Thumb, a subset of instructions used for embedded systems.
  - Load-Store architecture, meaning arithmetic operations can only occur between registers, data from memory must be loaded into a register first.
- RISC-V @riscv
  - An open source RISC instruction set architecture.
  - Contains different base instruction sets for 32-bit, 64-bit and 128-bit word sizes, along with extensions for features like multiplication and floating point.
  - Seperated into unprivileged instructions for most applications, and privileged for features like virtual memory meant to be used by operating systems and similar.

=== Existing Implementations

==== ASTRO-8

ASTRO-8 is a 16-bit computer design, and an accompanying emulator and assembler. The emulator supports many different IO methods, including a 108x108 px virtual display, keyboard and mouse input, and sound output with 4 channels. The architecture supports using multiple banks of memory, however only has 3 general purpose registers. The emulator is a desktop application that works alongside a separate assembler CLI program, however it only provides the display output and lacks any debugging features @astro8.

==== yasp

yasp is a web-based assembler development environment designed to make assembly programming more approachable. It simulates different hardware devices such as LEDs, buttons and potentiometers. The code writing process is interactive, featuring live error checking and helpful instruction hints when hovering over instructions. Debugging is a major focus of yasp, with support for breakpoints and stepping both forward and backwards through instructions, making it easier to analyze program behaviour. One drawback is that because it is web-based, it only manages to reach speeds of around \~25 KHz, however without any display facilities this isn't such an issue @yasp.

 #figure(caption: [The yasp user interface, showing the assembly code next to the debugger output.], image(width: 80%, "yasp.png"))

==== RARS

RARS (RISC-V Assembler and Runtime Simulator) is an assembler and simulator for the `riscv32` and `riscv64` instruction sets, with a goal of being an effective development environment for programmers getting started with RISC-V. RARS comes with a runtime that supports various syscalls, and debugging using breakpoints. The included assembler allows showing pseudo-instructions and the produced machine code side by side.

#figure(caption: [The RARS user interface.], image(width: 80%, "rars.png"))

== Objectives

+ _Assembler_
  + The assembler should be able to load an assembly file from the command line arguments.
    + If the given filename doesn't exist, or isn't valid UTF-8 the assembler should display an error and exit.
  + If no command line arguments are given, the assembler should show a help message.
  + The assembler should iterate through all of the lines of the program.
    + It should skip comments (lines beginning with a semicolon) and lines consisting only of whitespace.
    + It should parse each opcode mnemonic, then a comma separated list of operands.
    + If the mnemonic is unrecognized the program should display an error and exit.
    + It should treat the opcode and operands as case-insensitive.
    + It should be able to parse numeric literals of different bases (0xA3, 0b0111...).
    + If the given base for a numeric literal is invalid it should display an error and exit.
    + If an operand begins with a `%`, it should be treated as a register.
    + If an invalid register name is used, it should display and error and exit.
    + It should verify if the given operands are compatible with the opcode, otherwise it should display an error and exit.
    + If a label is defined (string followed by colon), it should insert the current position and label name into a symbol map.
    + If a label is used, it should insert the current position and label name into a symbol usages map.
    + There should be pseudo instructions to define raw data.
    + If an error occurs whilst assembling, the outputted error message should display the location in the source file where it occurred.
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
    + If a symbol isn't defined in any of the symbol maps, it should show an error and exit
    + It should insert the location of the symbol definition into the machine code at the usage address.
  + The linker should output the final machine code to a binary file given in the command line arguments.
+ _Emulator_
  + The emulator should be able to load a machine code file into memory.
  + The state of the CPU and memory should be able to be saved/loaded from a file.
  + The emulator should execute the instruction at the virtual program counter if it is unpaused or the user requested to step 1 instruction.
    + It should decode the instruction at the program counter.
    + If the decoded opcode is invalid, #strike([an interrupt should be raised]) the register state should be reset.
    + The decoded instruction should be executed, and the registers and memory should be updated accordingly.
  + The emulator should have a user interface with multiple panels inside.
    + These panels should be able to be moved, resized, opened and closed by the user.
  + There should be a window to control the current CPU state.
    + There should be a button to pause/resume the CPU execution based on current state.
    + There should be a button to step the CPU forward one cycle.
    + There should be a way to vary the execution speed of the CPU.
    + The execution speed should be able to be set to at least 1 MHz.
    + This window should display if the CPU is currently active or not.
    + This window should show the the decoded string of the last instruction.
  + There should be a window to display the CPU registers.
    + The register values should be editable by the user.
    + It should verify whether the inputted value is valid for the base the value is displayed in.
  + There should be a window to inspect the memory.
    + This window should display the memory address for each row.
    + The memory data should be displayed in hexadecimal.
    + The user should be able to edit individual bytes.
    + It should verify whether the input is valid hexadecimal.
    + The user should be able to jump directly to a specific address in the memory window.
  + There should be a window to view a virtual display.
    + The data for the display should be mapped to a region in the emulated memory.
    + #strike([With this window selected, any keyboard inputs should be sent to the CPU as interrupts.])
    + The window should show the color data and coordinates of the hovered pixel.
  + There should be a window to interact with a virtual serial port.
    + The user should be able to type in an a message, which will be then encoded as UTF-8 and queued to be sent to the CPU.
    + If the emulated program reads from a specific address, the value read should be popped from the queue.
    + If the emulated program writes to a specific address, it should be decoded as UTF-8 and displayed in the window.
+ The program should work on Windows, MacOS and Linux.

= Documented Design

== Project Structure

This project will contain the following 5 rust crates:
- `q16`: The library where most of the logic is implemented. This is so the emulation and assembly logic can be reused between the emulator and tests. This library also hosts the enums that define the values assigned for each opcode and register.
- `q16-asm`: The assembler CLI.
- `q16-ld`: The linker CLI. Used to link together multiple object files produced by the assembler.
- `q16-emu`: The emulator. A graphical application that can load machine code that has been linked and run programs interactively.
- `q16-tests`: An automated test runner that assembles and runs programs and compares the registers to expected outputs.

The project uses unstable APIs from Rust Nightly and was developed and tested on version `rustc 1.84.0-nightly (81eef2d36 2024-11-11)`.

The project code conforms to standard Rust conventions, and has been linted using `Clippy` @clippy and `rustfmt` to ensure a consistent code style.

== Libraries Used

#table(
  columns: 3,
  [*Name*], [*Purpose*], [*Link*],
  [`egui`], [Used for rendering UI widgets.], [https://crates.io/crates/egui],
  [`eframe`], [Backend for creating and rendering to a window.], [https://crates.io/crates/eframe],
  [`rfd`], [Used for opening native file dialog menus.], [https://crates.io/crates/rfd],
  [`strum`], [Utilities for working with enums. Used for converting enums to and from strings.], [https://crates.io/crates/strum],
  [`time`], [Used for working with timestamps.], [https://crates.io/crates/time],
  [`owo-colors`], [Used to print coloured text to the console], [https://crates.io/crates/owo-colors],
  [`regex`], [Regular expressions used to parse automated test files.], [https://crates.io/crates/regex]
)

== Project Overview

#figure(caption: [Overview of the projects structs and how they are linked.], image("overview.png"))

== Key Structures

#let snippet(src) = text(size: 10pt, raw(lang: "rust", src))

=== `Assembler`

Contains all of the logic necessary for assembling a file.

Members:
- #snippet("pub obj: Obj") - The output object file.
Methods:
- #snippet("pub fn assemble(&mut self, src: &str) -> Result<(), (usize, String)>") - Assemble an entire source file.
- #snippet("fn assemble_line(&mut self, line: &str) -> Result<(), String>") - Assemble a single line.
- #snippet("fn assemble_instr(&mut self, mnemonic: &str, operands: Vec<Operand>) -> Result<(), String>") - Output an instruction from the parsed line.

=== `Operand`

An enum for the different kinds of operands. Used whilst assembling.

Methods:
- #snippet("fn parse(s: &'a str) -> Result<Self, String>") - Parse the given string as an operand.
- #snippet("fn parse_literal(s: &str, radix: u32) -> Result<Self, String>") - Attempt to parse the given string as a literal operand, ignoring the first 2 characters if radix != 2.

=== `Obj`

Represents an object file.

Members:
- #snippet("pub labels: HashMap<String, u16>") - A map of label definitions to their addresses within `data`.
- #snippet("pub label_uses: Vec<(String, u16)>") - A list of the addresses where labels are used.
- #snippet("pub data: Vec<u8>") - The raw instruction data.

Methods:
- #snippet("pub fn load(data: &[u8]) -> Result<Self, String>") - Attempt to load an object from the given data. See @obj_format.
- #snippet("pub fn insert_label(&mut self, label: String) -> Result<(), String>") - Declare a label at the current position in `data`.
- #snippet("pub fn insert_label_usage(&mut self, label: String, offset: usize)") - Insert a label usage at the current position in `data` with a given offset.
- #snippet("pub fn emit_instr(&mut self, instr: Instruction)") - Write the given instruction to `data`.
- #snippet("pub fn extend(&mut self, other: Self) -> Result<(), String>") - Append another object file to self. Attempts to merge label declarations and errors if this fails.
- #snippet("pub fn out_obj(self) -> Vec<u8>") - Convert this object to a list of bytes ready to be written to a file in the format given in @obj_format.
- #snippet("pub fn out_bin(mut self) -> Result<Vec<u8>, String>") - Output the contents of this object to machine code. Attempts to resolve any label usages and errors if this fails.

=== `Emulator`

Responsible for emulation logic. Peripherals can be found in @emustate_struct

Members:
- #snippet("pub memory: Vec<u8>") - The emulators memory. Should always be 65536 bytes long.
- #snippet("pub registers: Registers") - Register state.

Methods:

- #snippet("pub fn new() -> Self") - Constructor for the CPU with zeroed memory and registers.
- #snippet("pub fn set_run(&mut self, run: bool)") - Set the run bit of the `sts` register.
- #snippet("pub fn running(&mut self) -> bool") - Get the run bit of the `sts` register.
- #snippet("pub fn cycle(&mut self) -> CycleOutput") - Execute one full cycle of the CPU. `CycleOutput` contains the decoded instruction and any memory loads/stores.
- #snippet("pub fn reset(&mut self)") - Zero the registers and memory
- #snippet("pub fn soft_reset(&mut self)") - Zero the registers only.
- #snippet("pub fn load_word(&self, addr: u16) -> u16") - Load a `u16` from `memory` at the given address. Handles overflow at the last byte.
- #snippet("pub fn load_byte(&self, addr: u16) -> u8") - Loads a `u8` from `memory` at the given address.
- #snippet("pub fn store_word(&mut self, addr: u16, x: u16)") - Write a `u16` to `memory` at the given address. Handles overflow at the last byte.
- #snippet("pub fn store_byte(&mut self, addr: u16, x: u8)") - Write a `u8` to `memory` at the given address.
- #snippet("pub fn save_state(&self) -> Vec<u8>") - Create a list of bytes containg the emulator state as defined in @emu_state_format.
- #snippet("pub fn from_state(mut data: Vec<u8>) -> Option<Self>") - Load the state from a list of bytes in the format given in @emu_state_format.

=== `CircularBuffer`

A circular queue. Used instead of `std::collections::VecDeque` for calculating the average elapsed cycle time.

Members:
- #snippet("buf: [T; N]") - The raw data. Is initially uninitialized and unsafe to access.
- #snippet("head: usize") - Head pointer.
- #snippet("len: usize") - Amount of elements in the queue.

Methods:
- #snippet("pub fn new() -> Self") - Constructor for an empty queue.
- #snippet("pub fn clear(&mut self)") - Empty the contents of the queue.
- #snippet("pub fn len(&self) -> usize") - Returns the amount of elements in the queue.
- #snippet("pub fn push(&mut self, item: T)") - Enqueue an element. Will overwrite the oldest element if the queue is full.
- #snippet("pub fn items(&self) -> &[T]") - Return the contents of the queue.


=== `ArgParser`

Utility for parsing command line arguments.

Members:
- #snippet("args: Vec<String>") - A list of all arguments that haven't been handled.

Methods:
- #snippet("pub fn from_env() -> Self") - Construct from the arguments passed to the CLI.
- #snippet("pub fn take_flag(&mut self, flag: &str) -> Option<String>") - Get the argument following `flag` if it exists. Removes both the flag and content from `args`.
- #snippet("pub fn remaining(self) -> Vec<String>") - Returns any unhandled arguments.

=== `EmuState` <emustate_struct>

Responsible for scheduling the CPU cycles and managing serial input.

Members:
- #snippet("emu: Emulator") - The internal emulator.
- #snippet("last_instr: Option<Instruction>") - The last decoded instruction.
- #snippet("target_speed: u64") - The target CPU frequency in Hertz.
- #snippet("time_history: CircularBuffer<Duration, 100_000>") - A circular buffer containing the time taken for the last 100000 cycles.
- #snippet("msg_log: Vec<(OffsetDateTime, String)>") - A log for messages to be shown to the user.
- #snippet("serial_in_queue: VecDeque<u8>") - A queue containing serial input that is yet to be sent to the CPU.
- #snippet("serial_out: Vec<u8>") - Any serial output from the CPU.

Methods:
- #snippet("pub fn load_binary<P: AsRef<Path>>(&mut self, path: P)") - Load a binary from the given filepath into position 0 in the emulators memory.
- #snippet("pub fn load_state<P: AsRef<Path>>(&mut self, path: P)") - Load the emulator state from the given filepath.
- #snippet("pub fn save_state<P: AsRef<Path>>(&mut self, path: P)") - Save the emulator state to the given file path.
- #snippet("pub fn cycle(&mut self)") - Cycle the CPU and process any IO events as necessary.
- #snippet("pub fn on_reset(&mut self)") - Reset the serial IO.
- #snippet("pub fn log(&mut self, msg: String)") - Write a log message.

=== `App`

Manages UI windows. Notably implements `eframe::App` which is required for UI rendering.

Members:
- #snippet("emu_state: Arc<Mutex<EmuState>>") - The internal emulator state. Wrapped in a mutex so it can be accessed from both the UI and emulation threads.
- #snippet("windows: Vec<Box<dyn Window>>") - List of the windows that can possibly be displayed.

Methods:
- #snippet("fn new(cc: &eframe::CreationContext) -> Self") - Initialize the application, spawning the emulation thread.
- #snippet("fn for_windows<F: FnMut(Arc<Mutex<EmuState>>, &mut dyn Window, &mut bool)>(&mut self, ctx: &egui::Context, mut f: F)") - Internal utility to run the given closure for every window.
- #snippet("fn file_button<P: Fn() -> Option<PathBuf> + Send + 'static, A: Fn(Arc<Mutex<EmuState>>, PathBuf) + Send + 'static>(&self, picker: P, action: A)") - Internal utility for the shared logic between file menu buttons. Spawns another thread to execute `picker` in order to not block the UI thread.
- #snippet("fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame)") - Called by `eframe` every frame. Renders the user interface.

=== `Window`

Trait (similar to an interface in other languages) implemented for every window struct. Each window struct is stored in a `Box`, which allows for dynamic dispatch. 

Methods:
- #snippet("fn build<'a>(&self, window: egui::Window<'a>) -> egui::Window<'a>") - Allows any modifications to be made to the `egui::Window` if necessary.
- #snippet("fn name(&self) -> &'static str") - Returns the name of the window. Should be kept constant (Not an associated constant due to how Rust deals with dynamic dispatch).
- #snippet("fn show(&mut self, state: &mut EmuState, ui: &mut egui::Ui)") - Ran every frame. Renders the UI for that window.

== Key Algorithms

=== Emulation thread

The emulation and UI rendering run on different threads in order to enable the emulation to run at speeds higher than the refresh rate of the user's display. This means that the `EmuState` must be wrapped in an `Arc` (reference counted pointer) to be sent across threads, and a `Mutex` to allow multiple threads to access the state by locking access to only one thread at a time. \
The scheduler keeps track of time to be carried forward, in the cases that the last cycle took too long to execute, for example when the UI thread locks the mutex during rendering. Implementing this raised the maximum achievable emulation speed from \~10 MHz to \~25 MHz on an Apple M2 processor (the speed is less stable at small timescales with this method, however at high speeds this is not noticeable).

#figure(caption: [The emulation thread scheduling algorithm], image(width: 80%, "emu_thread.png"))

=== Assemble file

#figure(caption: [Flowchart for assembling a file.], image(width: 80%, "asm_file.png"))

== File Formats

All values saved are stored in little endian and packed contiguously without padding.

=== Object File <obj_format>

All object files must begin with the bytes `[113, 49, 54]`, corresponding to "q16" in ASCII. \
This is followed by 2 tables containing label information. Each table starts with a 16-bit integer representing the number of entries in the table. The rest of the table contains null-terminated UTF-8 strings, each followed by a 16-bit integer. \
The first table is a map of label definitions to their address relative to the start of this files object code, and the second is a map containing label usages, and where the linker should insert the corresponding address into the machine code. \
The rest of the file contains the assembler output.

=== Emulator State File <emu_state_format>

The first 65,536 ($2^16 + 1$) bytes of the state file are the contents of the emulator memory. The rest of the file contains the registers, each represented as 16-bit integer, stored in the order defined in @regtable.

== UI Design

The UI will consist of windows that can be rearranged by the user and managed from a menu at the top of the screen. There will also be a File menu for loading/restoring the emulator state.

=== CPU State Window

This window allows the user to inspect and edit registers and control the how the CPU runs.
#figure(caption: [CPU state window mockup], image(width: 80%, "ui_state.png"))

=== Memory Window

The memory window is inspired by the designs of other hex editors, however without an ASCII section.
#figure(caption: [Memory editor window mockup], image(width: 80%, "ui_memory.png"))

== Demo Programs

To demonstrate the functionality, and teach the user how to operate the software, I plan to write sample programs that will be shipped with the final product.

=== Mandelbrot plotter

The Mandelbrot set is defined in the complex plane for each complex number $c$, where $f_c (z) = z^2 + c$, does not diverge to infinity when iterated starting at $z=0$. @mandelbrot It can be plotted by running the following steps for every pixel on the display.
+ Scale and transform the coordinate of the pixel from the (0,0)-(128,96) range down to something more suitable, for example (-2, -4)-(0.8, 4). The coordinates will need to be represented as fixed point binary since the architecture has no floating point capabilities.
+ Starting at $z=0$, compute $z^2 + c$, where the x and y coordinates are the real and imaginary components of $c$ respectively.
+ If $|z|>4$, then break from the loop, as we know we will diverge to infinity now, otherwise repeat the computation of $z$ from the last step.
+ If the number of iterations exceeds a preset value, for example 50, then break from the also break from the loop as $f_c(z)$ is unlikely to diverge at this point.
+ Assign the pixel a colour based on the number of iterations before breaking.

=== Game of Life

Conway's Game of Life is a cellular automaton where the evolution of game is only determined by its initial state. The system is represented by a two-dimensional grid of cells, where each cell is either _alive_ or _dead_. The _neighbours_ of a cell are the 8 surrounding cells. With each step of the simulation, the following rules are applied for each cell @cgol.
+ Any live cells with less than 2 neighbours die (_underpopulation_).
+ Any live cells with 2 or 3 neighbours live.
+ Any live cells with more than 3 neighbours die (_overpopulation_).
+ Any dead cells with 3 neighbours become alive (_reproduction_).
The next generation of the game is first computed in a seperate buffer, before being copied to the screen. This ensures that the rules are applied simultaneously to every cell at once.

=== Fibonacci sequence

The Fibonacci sequence is trivial to compute. Given the starting conditions $F_0 = 0$ and $F_1 = 1$, the nth term in the sequence is given by $F_n = F_(n-1) + F_(n-2)$.
\ The relevant part of this program is outputting integers to the serial console as ASCII. To print the value $x$, the following steps are performed.
+ Calculate $x space % space 10$. Add the ASCII value of the character '0' to this and push it to the stack. Finally divide $x$ by 10.
+ Repeat step 1 until $x$ reaches.
+ Pop each element from the stack and write it to the serial port. Repeat until there are no characters left.
Here the stack is used to reverse the digits from ascending to descending order.

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
) <regtable>
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
The branching instructions use the value in the `sts` register and are designed to be used with the `cmp` pseudo-instruction, although this is not necessarily required. For the memory and branching operations, either one of `r1` or `imm` can be omitted.

=== Pseudo Instructions

Many assembly instructions are implemented using other instructions. The `.db` and `.dw` instructions can also be used to insert values directly into the machine code, and the `.skip` instruction to insert n 0s into the machine code.

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

// TODO complete
#table(
  columns: (auto, 0.4fr, 1fr),
  [*Group*], [*Skill*], [*Location*], 
  [A], [Hash Tables], [`q16/src/obj.rs`],
  [A], [List operations], [Everywhere where `Vec` is used],
  [A], [Queue operations], [`emu/src/main.rs:172`\ `emu/src/ui/serial.rs`\ `q16/src/util.rs:19`],
  [A], [Stack operations], [`demos/fibonacci.asm:33`\ `tests/auto/programs/factorial.asm`],
  [A], [Interfaces], [`emu/src/ui/mod.rs:16`\ `emu/src/main.rs:56`],
  [A], [Recursive algorithms], [`tests/src/main.rs:45`\ `tests/auto/programs/factorial.asm`],
  [B], [File access], [`asm/src/main.rs`\ `ld/src/main.rs`\ `emu/src/main.rs:144`],
  [B], [Bubble Sort], [`tests/auto/programs/bubble_sort.asm`],
  [B], [Records], [All `struct` definitions],
)

== Source Code

#let sourcecode(lang: "rust", desc: "",  path) = [
  // #set par(justify: false)
  === #raw(path)
  #desc
  #zebraw(lang: false, text(10pt, raw(lang: lang, block: true, read("../" + path))))
]

#sourcecode(desc: "Main definitions for the instruction set and instruction parsing.", "q16/src/lib.rs")
#sourcecode(desc: "Main assembler implementation.", "q16/src/asm.rs")
#sourcecode(desc: "Object file manipulation.", "q16/src/obj.rs")
#sourcecode(desc: "Main emulator implementation.", "q16/src/emu.rs")
#sourcecode(desc: "Utility functions and types.", "q16/src/util.rs")
#sourcecode(desc: "Assembler CLI entry point.", "asm/src/main.rs")
#sourcecode(desc: "Linker CLI entry point.", "ld/src/main.rs")
#sourcecode(desc: "Emulator application initilization.", "emu/src/main.rs")
#sourcecode(desc: "Emulator window organization.", "emu/src/ui/mod.rs")
#sourcecode(desc: "CPU state window.", "emu/src/ui/cpu_state.rs")
#sourcecode(desc: "Memory editor window.", "emu/src/ui/memory.rs")
#sourcecode(desc: "Virtual display window.", "emu/src/ui/display.rs")
#sourcecode(desc: "Serial console window.", "emu/src/ui/serial.rs")
#sourcecode(desc: "Message log window.", "emu/src/ui/log.rs")
#sourcecode(desc: "Automated test runner.", "tests/src/main.rs")
#sourcecode(desc: "Entry point for demo applications.", lang: "asm", "demos/base.asm")
#sourcecode(desc: "Draws the mandelbrot set to the virtual display.", lang: "asm", "demos/mandelbrot.asm")
#sourcecode(desc: "Simulates Conway's Game of Life on the virtual display.", lang: "asm", "demos/gameoflife.asm")
#sourcecode(desc: "Prints the first 24 Fibonacci terms to the serial console.", lang: "asm", "demos/fibonacci.asm")
#sourcecode(desc: "Echos user input in the serial console.", lang: "asm", "demos/echo.asm")

#set page(flipped: true)
= Testing

== Testing table

The emulator UI tests are evidenced in a seperate video file, with timestamps referenced here.

#let pass(x) = text(fill: green, x);

#table(
  columns: (auto, 1fr, 1fr, 2fr, auto),
  table.header([*Number*\ #pass([Pass])/#text(fill: red, [Fail])], [*Input*], [*Expected Output*], [*Evidence*], [*Objectives*]),
  pass([1.1]), [Assembler CLI arguments incomplete.], [Help message is displayed.], [#image("tests/1_1.png")], [1.2],
  pass([1.2]), [Invalid UTF-8 source file.], [Error message], [#image("tests/1_2.png")], [1.1.1],
  pass([1.3]), [Non existent source file.], [Error message], [#image("tests/1_3.png")], [1.1.1],
  pass([1.4]), [Invalid mnemonic used in the source file.], [Error message and location.], [#image("tests/1_4.png")], [1.3.3\ 1.3.13],
  pass([1.5]), [Invalid register used in the source file.], [Error message and location.], [#image("tests/1_5.png")], [1.3.8\ 1.3.13],
  pass([1.6]), [Invalid base used for integer literal.], [Error message and location.], [#image("tests/1_6.png")], [1.3.5\ 1.3.6\ 1.3.13],
  pass([1.7]), [Invalid digit used for integer literal.], [Error message and location.], [#image("tests/1_7.png")], [1.3.5\ 1.3.6\ 1.3.13],
  pass([1.8]), [Conflicting label definitions in source file.], [Error message, location and problematic label name.], [#image("tests/1_8.png")], [1.3.10\ 1.3.13],
  pass([1.9]), [Source file containing mnemonics, registers and literals with varying case.], [Compiled object file.], [#image("tests/1_9.png")], [1.3.4\ 1.3.5],
  pass([1.10]), [Valid source file.], [Object file outputted to correct path (contents are verified by other modules).], [#image("tests/1_10.png")], [1.4],
  pass([2.1]), [Linker CLI arguments incomplete.], [Help message is displayed.], [#image("tests/2_1.png")], [2.1],
  pass([2.2]), [Input object with invalid header.], [Error message identifying problematic file.], [#image("tests/2_2.png")], [2.1\ 2.2.1],
  pass([2.3]), [Non existent input object.], [Error message.], [#image("tests/2_3.png")], [2.1],
  pass([2.4]), [Linking objects that both contain labels with the same name], [Error message displaying problematic label name.], [#image("tests/2_4.png")], [2.2.3],
  pass([2.5]), [Linking objects that contain labels that are not defined anywhere.], [Error message displaying problematic label name.], [#image("tests/2_5.png")], [2.3.1],
  pass([2.6]), [Valid input objects.], [Output at correct path.], [#image("tests/2_6.png")], [2.4],
  pass([3.1]), [Clicking "File>Load Binary" and selecting a file < 65 KiB.], [The file is loaded into the emulators memory.], [Video timestamp 00:01], [3.1],
  pass([3.2]), [Clicking "File>Load Binary" and selecting a file > 65 KiB.], [An error is displayed.], [Video timestamp 00:06], [3.1],
  pass([3.3]), [Clicking "File>Save State".], ["The save file should be created at the user provided location."], [Video timestamp 00:13], [3.2],
  pass([3.4]), [Clicking "File>Load State" and selecting a state file.], ["The state of the CPU should be restored from the file."], [Video timestamp 00:19], [3.2],
  pass([3.5]), [Clicking CPU play button.], [The emulation should begin at the set speed. The button should update to a stop symbol.], [Video timestamp 00:32], [3.5.1\ 3.5.5],
  pass([3.6]), [Clicking CPU stop button.], [The emulation should stop. The button should update to a play symbol.], [Video timestamp 00:35], [3.5.1\ 3.5.5],
  pass([3.7]), [Clicking CPU step instruction button.], [One cycle should be processed.], [Video timestamp 00:42], [3.5.2],
  pass([3.8]), [Varying the emulation speed while CPU is running.], [The observed speed of cycles should change.], [Video timestamp 00:35], [3.5.3],
  pass([3.10]), [Setting the emulation speed to a value greater than 1 MHz.], [The observed speed should be at least 1 MHz.], [Video timestamp 00:40], [3.5.4],
  pass([3.11]), [Typing valid hexadecimal into the register inputs.], [The value should be updated.], [Video timestamp 00:49], [3.6],
  pass([3.12]), [Typing invalid hexadecimal into the register inputs.], [The invalid value should not be saved.], [Video timestamp 00:52], [3.6.2],
  pass([3.13]), [Typing valid binary into the status register input.], [The value should be updated.], [Video timestamp 00:55], [3.6.2],
  pass([3.14]), [Typing invalid binary into the status register input.], [The invalid value should not be saved.], [Video timestamp 01:10], [3.6.2],
  pass([3.15]), [Typing valid hexadecimal into the memory cells.], [The values should be stored.], [Video timestamp 01:04], [3.7.1\ 3.7.2\ 3.7.3],
  pass([3.16]), [Typing invalid hexadecimal into the memory cells.], [The invalid value should not be stored.], [Video timestamp 01:09], [3.7.4],
  pass([3.17]), [Typing valid hexadecimal into the 'Jump To' input.], [The memory window should scroll to the correct address.], [Video timestamp 01:11], [3.7.5],
  pass([3.20]), [Typing invalid hexadecimal into the 'Jump To' input.], [The value should not be updated and the window should not scroll.], [Video timestamp 01:15], [3.7.5],
  pass([3.21]), [Run a program that uses the display.], [The contents of the display should be updated.], [Video timestamp 01:24], [3.8.1],
  pass([3.22]), [Hover pixels on the display.], [The color codes and coordinates of the hovered pixel are displayed.], [Video timestamp 01:25], [3.8.3],
  pass([3.23]), [Type a message into the serial console.], [The users input is displayed.], [Video timestamp 01:40], [3.9.1],
  pass([3.24]), [Pressing enter with a message in the serial console input.], [The input should be sent to the CPU.], [Video timestamp 01:45], [3.9.1\ 3.9.2],
  pass([3.25]), [Using a program that outputs to the serial console.], [The output is visible in the serial console window.], [Video timestamp 01:45], [3.9.3],
)

#set page(flipped: false)

== Automated tests

The `q16-tests` package allows automatically assembling and emulating programs, and verifying the outputs based on comments in the source file. This is used for testing the core functionality. Every time the CPU is halted, the test runner will verify the contents of the registers with the next occurrence of a string with the pattern "`;assert r3=36 r4=92 ...`".

These tests cover all objectives under 1.3, 2.2.2, 2.3.2, 2.2.4 and 3.3.

#let testcode(path) = [
  ==== #raw(path)
  #zebraw(lang: false, text(10pt, raw(lang: "asm", block: true, read("../tests/auto/" + path))))
]

#testcode("literals.asm")
#testcode("branching.asm")
#testcode("registers.asm")
#testcode("invalid_instr.asm")
#testcode("mem/loads.asm")
#testcode("mem/stores.asm")
#testcode("mem/boundary.asm")
#testcode("instructions/add.asm")
#testcode("instructions/sub.asm")
#testcode("instructions/mul.asm")
#testcode("instructions/div.asm")
#testcode("instructions/rem.asm")
#testcode("instructions/and.asm")
#testcode("instructions/or.asm")
#testcode("instructions/xor.asm")
#testcode("programs/factorial.asm")
#testcode("programs/bubble_sort.asm")

#figure(image(width: 80%, "autotests.png"))

== Rust unit tests

Unit tests for internal utilities can be found at `q16/src/util.rs:100`. These all pass.
#figure(image(width: 80%, "unittests.png"))

= Evaluation

Overall, I believe my project achieves the goals set out by the client and has met all of the agreed upon objectives. Due to my original interrupt design being awkward to work with, I agreed with the client to abandon objective 3.8.2 (keyboard input interrupts), and introduced objective 3.9 (the serial console). The serial console is much more friendly for the user to work with as they do not need to worry about font rendering on the display to output text.

== Client Feedback

The client found the UI simple and intuitive to use, and agreed that all objectives were met. They said the program helps them achieve their goals, and that reading the provided assembly snippets taught them alot about how programs are executed at a low level. Giving the program to other computer science students to test, they agreed, stating "The ability for windows to be moved and resized really improves the workflow for anyone looking to use the app". They also commented that "The simulation is surprisingly fast given it is being emulated", and that the ability to write graphical programs from assembly was very intriguing.

== Possible improvements

As this was designed as a tool for learning, it may have been a better idea to have the assembler integrated within the emulator's user interface, which would speed up the development cycle dramatically as users would be able to immediately test their changes. Due to the modular design of my codebase, adding this in the future would not require too much extra effort. Additionally support for extra peripherals could be added, for example secondary storage or hardware timers.

= Bibliography

#bibliography(title: none, full: true, "bibliography.yml")
