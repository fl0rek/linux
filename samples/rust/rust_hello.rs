//! Rust Module
//!

use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use kernel::io_buffer::{IoBufferReader, IoBufferWriter};
use kernel::prelude::*;
use kernel::str::CString;
use kernel::sync::{Ref, RefBorrow};
use kernel::{file, miscdev};

module! {
    type: RustMiscdev,
    name: "rust_hello",
    author: "florek",
    description: "Rust hello world",
    license: "GPL v2",
}

struct SharedState {
    already_opened: AtomicBool,
    count: AtomicU64,
}

struct RustMiscdev {
    _dev: Pin<Box<miscdev::Registration<RustFile>>>,
}

impl kernel::Module for RustMiscdev {
    fn init(name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Char device (init): {name}\n");

        let state = Ref::try_new(SharedState {
            already_opened: AtomicBool::new(false),
            count: AtomicU64::new(0),
        })?;

        let miscdev_reg = miscdev::Registration::new_pinned(fmt!("{name}"), state)?;

        Ok(Self { _dev: miscdev_reg })
    }
}

impl Drop for RustMiscdev {
    fn drop(&mut self) {
        pr_info!("Rust Hello, World! (exit)\n");
    }
}

struct RustFile;

#[vtable]
impl file::Operations for RustFile {
    type OpenData = Ref<SharedState>;
    type Data = Ref<SharedState>;

    fn open(shared: &Ref<SharedState>, _file: &file::File) -> Result<Ref<SharedState>> {
        if shared
            .already_opened
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            Ok(shared.clone())
        } else {
            Err(EBUSY)
        }
    }

    fn release(shared: Ref<SharedState>, _: &file::File) {
        shared.already_opened.store(false, Ordering::Release);
    }

    fn read(
        shared: RefBorrow<'_, SharedState>,
        _: &file::File,
        data: &mut impl IoBufferWriter,
        offset: u64,
    ) -> Result<usize> {
        if data.is_empty() || offset != 0 {
            return Ok(0);
        }

        let msg = CString::try_from_fmt(fmt!(
            "written {} bytes\n",
            shared.count.load(Ordering::Acquire)
        ))?;

        data.write_slice(msg.as_bytes())?;

        Ok(msg.len())
    }

    fn write(
        shared: RefBorrow<'_, SharedState>,
        _: &file::File,
        data: &mut impl IoBufferReader,
        _: u64,
    ) -> Result<usize> {
        let len = data.len();
        shared.count.fetch_add(len.try_into()?, Ordering::AcqRel);
        Ok(len)
    }
}

