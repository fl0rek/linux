// SPDX-License-Identifier: GPL-2.0

//! Rust file system sample.

use kernel::prelude::*;
use kernel::{c_str, fs};

module_fs! {
    type: RustFs,
    name: "rust_fs",
    author: "mikolaj@florkiewicz.me",
    license: "GPL",
}

struct RustFs {
    _dev: Pin<Box<fs::Registration>>,
}

#[derive(Default)]
struct State {
    verbose: bool
}


#[vtable]
impl fs::Context<Self> for RustFs {
    type Data = Box<State>;

    kernel::define_fs_params! { Box<State>,
        {flag, "verbose", |s, v| { s.verbose = v; Ok(()) } },
    }

    fn try_new() -> Result<Self::Data> {
        pr_info!("context created!\n");

        Ok(Box::try_new(State::default())?)
    }
}

impl fs::Type for RustFs {
    type Context = Self;
    const SUPER_TYPE: fs::Super = fs::Super::Independent;
    const NAME: &'static CStr = c_str!("rustfs");
    const FLAGS: i32 = fs::flags::USERNS_MOUNT;

    fn fill_super(_data: Box<State>, sb: fs::NewSuperBlock<'_, Self>) -> Result<&fs::SuperBlock<Self>> {
        let sb = sb.init(
            (),
            &fs::SuperParams {
                magic: 0x21372137,
                ..fs::SuperParams::DEFAULT
            },
        )?;
        let sb = sb.init_root()?;
        Ok(sb)
    }
}

