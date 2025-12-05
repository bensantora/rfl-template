pub fn generate(module_name: &str, description: &str) -> String {
    format!(
        r#"# {module_name}

{description}

## Building

This module requires a Linux kernel source tree with Rust support enabled.

1. Link this directory into your kernel source tree:
   ```bash
   ln -s $(pwd) /path/to/linux/samples/rust/{module_name}
   ```

2. Enable the module in your kernel config:
   ```bash
   cd /path/to/linux
   make LLVM=1 menuconfig
   # Navigate to: Kernel hacking -> Sample kernel code -> Rust samples
   # Enable your module
   ```

3. Build the module:
   ```bash
   make LLVM=1 samples/rust/{module_name}.ko
   ```

## Loading

```bash
sudo insmod {module_name}.ko
dmesg | tail  # Check kernel log
sudo rmmod {module_name}
```

## License

See the SPDX identifier in the source code.
"#,
        module_name = module_name,
        description = description
    )
}
