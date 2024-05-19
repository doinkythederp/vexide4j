#![no_main]
#![no_std]
#![feature(error_in_core)]

extern crate alloc;

use alloc::{boxed::Box, sync::Arc, vec::Vec};
use core::{error::Error, time::Duration};

use rjvm_vm::{io::JvmIo, vm::Vm};
use unix_path::Path;
use vexide::{
    core::{
        io::{self, ErrorKind},
        time::Instant,
    },
    prelude::*,
};

struct VexideJvmIo;

impl JvmIo for VexideJvmIo {
    fn duration_since_epoch(&self) -> core::time::Duration {
        Duration::from_micros(unsafe { vex_sdk::vexSystemPowerupTimeGet() })
    }

    fn is_dir(&self, path: &Path) -> bool {
        false
    }

    fn exists(&self, path: &Path) -> bool {
        false
    }

    fn read(&self, path: &Path) -> Result<Vec<u8>, io::Error> {
        Err(ErrorKind::NotFound.into())
    }
}

#[vexide::main]
async fn main(peripherals: Peripherals) -> Result<(), Box<dyn Error>> {
    const MEMORY_80_MB: usize = 80 * 1024 * 1024;
    let mut jvm = Vm::new(MEMORY_80_MB, Arc::new(VexideJvmIo));
    Ok(())
}
