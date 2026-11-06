// SPDX-License-Identifier: GPL-2.0

//! Rust out-of-tree sample

use kernel::prelude::*;
use kernel::debugfs;

module! {
    type: RustOutOfTree,
    name: "rust_out_of_tree",
    authors: ["Rust for Linux Contributors"],
    description: "Rust out-of-tree sample",
    license: "GPL",
}

struct RustOutOfTree {
    file: Pin<KBox<debugfs::File<u32>>>,
}

impl kernel::Module for RustOutOfTree {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust out-of-tree sample (init)\n");

        let dir = debugfs::Dir::new(c"bend");
        let file = KBox::pin_init(dir.read_only_file(c"foo", 42), GFP_KERNEL)?;

        Ok(RustOutOfTree { file })
    }
}

impl Drop for RustOutOfTree {
    fn drop(&mut self) {
        pr_info!("Rust out-of-tree sample (exit)\n");
    }
}
