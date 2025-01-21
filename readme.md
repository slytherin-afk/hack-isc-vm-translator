# HACK VM Translator

An **translator** that converts the **VM instruction set** defined in the `nand2tetris` course into assembly, according to the Hack computer hardware specifications.

---

## Overview

This project provides an compiler/translator for the **Hack Language VM Stack Machine**. It takes an `.vm` file as input, containing vm instructions, and produces an equivalent `.asm`.

The Hack instruction set and hardware are defined in the **Nand2Tetris** course. This project implements the translation from vm instructions to assembly code.

---

## Features

- Supports commands
  - Arithmetic
  - Memory Access
- Ignores comments and whitespace for clean processing.
- Outputs clean, assembly.

---

## Input and Output

1. **Input**: A `.vm` file containing VM Stack machine instructions.  
   Example:

   ```vm
   push constant 10
   pop local 0
   push constant 21
   push constant 22
   pop argument 2
   pop argument 1
   push constant 36
   pop this 6
   push constant 42
   push constant 45
   pop that 5
   pop that 2
   push constant 510
   ```

2. **Output**: A `.asm` file containing assembly instructions.  
   Example:

   ```asm
   @2
   D=A
   @3
   D=D+A
   @0
   M=D
   ```

---

## Usage

### 1. Build the Compiler

To build the compiler, compile the source code.

```bash
cargo build
```

### 2. Run the Compiler

Run the executable and provide the input `.vm` file:

```bash
./hack-isc-vm-translator input.vm -o output.asm
```

The compiler will generate a file with the assembly instructions.

---

## References

- [Nand2Tetris](https://www.nand2tetris.org/)
