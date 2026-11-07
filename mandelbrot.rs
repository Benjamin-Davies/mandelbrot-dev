// SPDX-License-Identifier: GPL-2.0

//! Device driver that renders a Mandelbrot set

use core::ops::Add;
use core::ops::Div;
use core::ops::Mul;
use core::ops::Sub;
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

const WIDTH: usize = 80;
const HEIGHT: usize = 36;
const PALLETE: &[u8] = b"          ..++**#";

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
        _misc: &MiscDeviceRegistration<Self>,
    ) -> Result<Self::Ptr> {
        Ok(KBox::new(MandelbrotFile::default(), GFP_KERNEL)?)
    }

    fn read_iter(
        kiocb: Kiocb<'_, Self::Ptr>,
        iov: &mut IovIterDest<'_>,
    ) -> Result<usize> {
        let this = kiocb.file();
        if this.consumed.fetch_or(true, Ordering::AcqRel) {
            return Ok(0);
        }

        let mut row = [0; WIDTH + 1];
        let mut total = 0;
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                row[j] = get_char(i, j);
            }
            row[WIDTH] = b'\n';

            total += iov.copy_to_iter(&row);
        }

        return Ok(total);
    }
}

fn get_char(i: usize, j: usize) -> u8 {
    let x0 = 247 * ONE * j / (100 * WIDTH) - 200 * ONE / 100;
    let y0 = 224 * ONE * i / (100 * HEIGHT) - 112 * ONE / 100;
    let mut x = ZERO;
    let mut y = ZERO;
    let mut k = 0;
    while x * x + y * y <= 4 * ONE && k < PALLETE.len() - 1 {
        (x, y) = (x * x - y * y + x0, 2 * x * y + y0);
        k += 1;
    }

    PALLETE[k]
}

const FRAC: u32 = 24;
const ZERO: Fixed = Fixed(0);
const ONE: Fixed = Fixed(1 << FRAC);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Fixed(i64);

impl Add for Fixed {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Fixed {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl Mul for Fixed {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self((self.0 * rhs.0) >> FRAC)
    }
}

impl Mul<usize> for Fixed {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self {
        Self(self.0 * rhs as i64)
    }
}

impl Mul<Fixed> for usize {
    type Output = Fixed;

    fn mul(self, rhs: Fixed) -> Fixed {
        Fixed(self as i64 * rhs.0)
    }
}

impl Div<usize> for Fixed {
    type Output = Self;

    fn div(self, rhs: usize) -> Self {
        Self(self.0 / rhs as i64)
    }
}
