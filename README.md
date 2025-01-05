# Micron (WIP)

Micron is a lightweight interpreted programming language written in Rust.

Additional library files are written in Ruby.

## Build and Run Micron

Rust's default package manager and build system, Cargo, does not quite fit the needs of this project. However, it does fit some of the use cases so it is used where necessary.

Additionally, a program like Makefile running through Cargo is redundant and still is not quite what is required for this project.

Micromake is a C program that houses the primary build tools for the project instead, both running and building the project. This C program is run through a primitive Makefile just for compilation purposes.

### Build and Run Shortcuts
#### Cargo
- Clean the project: `cargo clean`

#### Micromake 
- Build Micromake: `make`
- Clean Micromake: `make clean`
- Run Micromake: `\micromake <program_file>.mcrn`
- Run Micromake (In Release): `\micromake -r <program_file>.mcrn`