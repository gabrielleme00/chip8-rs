# Chip-8 Interpreter and Disassembler in Rust (WIP)

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.73%2B-orange.svg)](https://www.rust-lang.org)

This is a WIP Chip-8 interpreter and disassembler implemented in Rust. Chip-8 is an interpreted programming language used on a variety of vintage computers, and this project allows you to run and disassemble Chip-8 programs.

## Features

- **Interpreter**: Run Chip-8 programs on your computer with this interpreter.
- **[WIP] Disassembler**: Disassemble Chip-8 programs to see their assembly code.
- **[WIP] User-Friendly Interface**: A simple and intuitive command-line interface for both interpreter and disassembler modes.
- **[WIP] Cross-Platform**: Only tested on Linux.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed on your system.

### Installation

Clone this repository and navigate to the project directory:

```bash
git clone https://github.com/gabrielleme00/chip8-rs.git
cd chip8-rs
```

Build the project using Cargo:

```bash
Copy code
cargo build --release
```

## Usage

To run a Chip-8 program, use the following command:

```bash
cargo run --release --bin interpreter path/to/rom.ch8
```

Replace path/to/rom.ch8 with the path to your Chip-8 program ROM.

## License
This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments
Special thanks to the Chip-8 community and the Rust programming language for making this project possible.
Cowgod's documentation was specially useful.

## Contributing
Contributions are welcome! If you have any suggestions or improvements, please open an issue or create a pull request.

Happy Chip-8 programming!
