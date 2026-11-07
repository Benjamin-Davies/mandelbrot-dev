// SPDX-License-Identifier: GPL-2.0

//! Device driver that renders a Mandelbrot set

use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;

use kernel::prelude::*;

use kernel::fs::Kiocb;
use kernel::fs::file::File;
use kernel::iov::IovIterDest;
use kernel::miscdevice::MiscDevice;
use kernel::miscdevice::MiscDeviceOptions;
use kernel::miscdevice::MiscDeviceRegistration;

module! {
    type: Mandelbrot,
    name: "mandelbrot",
    authors: ["Ben Davies"],
    description: "Device driver that renders a Mandelbrot set",
    license: "GPL",
}

const WIDTH: usize = 40;
const HEIGHT: usize = 10;

struct Mandelbrot {
    _dev: Pin<KBox<MiscDeviceRegistration<MandelbrotDevice>>>,
}

struct MandelbrotDevice;

#[derive(Default)]
struct MandelbrotFile {
    consumed: AtomicBool,
}

impl kernel::Module for Mandelbrot {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        let _dev = KBox::pin_init(
            MiscDeviceRegistration::register(
                MiscDeviceOptions { name: c"mandelbrot" },
            ),
            GFP_KERNEL,
        )?;

        Ok(Mandelbrot { _dev })
    }
}

#[vtable]
impl MiscDevice for MandelbrotDevice {
    type Ptr = KBox<MandelbrotFile>;

    const HAS_READ_ITER: bool = true;

    fn open(
        _file: &File,
        misc: &MiscDeviceRegistration<Self>,
    ) -> Result<Self::Ptr> {
        Ok(KBox::new(MandelbrotFile::default(), GFP_KERNEL)?)
    }

    fn read_iter(
        mut kiocb: Kiocb<'_, Self::Ptr>,
        iov: &mut IovIterDest<'_>,
    ) -> Result<usize> {
        let this = kiocb.file();
        if this.consumed.fetch_or(true, Ordering::AcqRel) {
            return Ok(0);
        }

        let mut row = [0; WIDTH + 1];
        let mut total = 0;
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if (x + y) % 2 == 0 {
                    row[x] = b'#';
                } else {
                    row[x] = b'.';
                }
            }
            row[WIDTH] = b'\n';

            total += iov.copy_to_iter(&row);
        }

        return Ok(total);
    }
}
