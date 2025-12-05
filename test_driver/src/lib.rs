// SPDX-License-Identifier: GPL-2.0

//! A test kernel driver

use kernel::prelude::*;

module! {
    type: TestDriver,
    name: "test_driver",
    author: "Test Author",
    description: "A test kernel driver",
    license: "GPL-2.0",
}

struct TestDriver;

impl kernel::Module for TestDriver {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("test_driver: Module loaded\n");
        Ok(TestDriver)
    }
}

impl Drop for TestDriver {
    fn drop(&mut self) {
        pr_info!("test_driver: Module unloaded\n");
    }
}
