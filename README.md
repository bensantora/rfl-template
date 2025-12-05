# rfl-template

A CLI tool to scaffold Rust for Linux (RFL) kernel modules with proper boilerplate and conventions.

## Features

- ✅ Generates complete kernel module structure
- ✅ Follows RFL conventions and best practices
- ✅ Includes proper SPDX headers and licensing
- ✅ Creates Kbuild integration files
- ✅ Validates module names
- ✅ Provides build instructions

## Installation

```bash
cargo install --path .
```

Or build from source:

```bash
cargo build --release
# Binary will be in target/release/rfl-template
```

## Usage

### Create a New Module

```bash
rfl-template new my_driver \
  --author "Your Name" \
  --description "My awesome kernel driver" \
  --license "GPL-2.0"
```

### Minimal Usage

```bash
rfl-template new my_driver
```

This will use default values:
- Author: "Unknown Author"
- Description: "A Rust for Linux kernel module"
- License: "GPL-2.0"

## Generated Structure

```
my_driver/
├── Cargo.toml          # Rust package configuration
├── Kbuild              # Kernel build system integration
├── README.md           # Build and usage instructions
└── src/
    └── lib.rs         # Module implementation
```

## Generated Code Example

The tool generates a basic kernel module with:

```rust
// SPDX-License-Identifier: GPL-2.0

//! My awesome kernel driver

use kernel::prelude::*;

module! {
    type: MyDriver,
    name: "my_driver",
    author: "Your Name",
    description: "My awesome kernel driver",
    license: "GPL-2.0",
}

struct MyDriver;

impl kernel::Module for MyDriver {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("my_driver: Module loaded\n");
        Ok(MyDriver)
    }
}

impl Drop for MyDriver {
    fn drop(&mut self) {
        pr_info!("my_driver: Module unloaded\n");
    }
}
```

## Module Name Requirements

Module names must:
- Start with a lowercase letter
- Contain only lowercase letters, numbers, and underscores
- Be valid Rust identifiers

**Valid**: `my_driver`, `test_module`, `driver2`  
**Invalid**: `MyDriver`, `my-driver`, `2driver`

## Building Generated Modules

Generated modules require a Linux kernel source tree with Rust support enabled.

See the generated `README.md` in each module for detailed build instructions.

## Options

```
Usage: rfl-template new <NAME> [OPTIONS]

Arguments:
  <NAME>  Name of the kernel module

Options:
  -a, --author <AUTHOR>            Author name
  -d, --description <DESCRIPTION>  Module description
  -l, --license <LICENSE>          License [default: GPL-2.0]
  -h, --help                       Print help
```

## Supported Licenses

Common kernel-compatible licenses:
- `GPL-2.0` (default)
- `GPL-2.0-only`
- `GPL-2.0-or-later`
- `MIT`
- `Apache-2.0`
- `BSD-3-Clause`

## Examples

### Character Device Driver

```bash
rfl-template new char_device \
  --author "John Doe" \
  --description "A character device driver" \
  --license "GPL-2.0"
```

### Network Driver

```bash
rfl-template new net_driver \
  --author "Jane Smith" \
  --description "A network device driver"
```

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

MIT OR Apache-2.0

## Resources

- [Rust for Linux](https://rust-for-linux.com/)
- [RFL Documentation](https://rust-for-linux.github.io/docs/)
- [Linux Kernel Documentation](https://www.kernel.org/doc/html/latest/)

---

**Made for the Rust for Linux community** 🦀🐧
