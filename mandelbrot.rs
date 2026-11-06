// SPDX-License-Identifier: GPL-2.0

//! Device driver that renders a Mandelbrot set

use kernel::prelude::*;
use kernel::debugfs;

module! {
    type: Mandelbrot,
    name: "mandelbrot",
    authors: ["Ben Davies"],
    description: "Device driver that renders a Mandelbrot set",
    license: "GPL",
}

struct Mandelbrot {
    _file: Pin<KBox<debugfs::File<u32>>>,
}

impl kernel::Module for Mandelbrot {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Mandelbrot (init)\n");

        let dir = debugfs::Dir::new(c"bend");
        let _file = KBox::pin_init(dir.read_only_file(c"foo", 42), GFP_KERNEL)?;

        Ok(Mandelbrot { _file })
    }
}

impl Drop for Mandelbrot {
    fn drop(&mut self) {
        pr_info!("Mandelbrot (exit)\n");
    }
}
