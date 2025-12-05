# test_driver

A test kernel driver

## Building

This module requires a Linux kernel source tree with Rust support enabled.

1. Link this directory into your kernel source tree:
   ```bash
   ln -s $(pwd) /path/to/linux/samples/rust/test_driver
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
   make LLVM=1 samples/rust/test_driver.ko
   ```

## Loading

```bash
sudo insmod test_driver.ko
dmesg | tail  # Check kernel log
sudo rmmod test_driver
```

## License

See the SPDX identifier in the source code.
