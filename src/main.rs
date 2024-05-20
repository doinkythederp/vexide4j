#![no_main]
#![no_std]
#![feature(error_in_core)]

extern crate alloc;

use alloc::{boxed::Box, format, string::String, sync::Arc, vec, vec::Vec};
use core::{fmt::Debug, time::Duration};

use include_dir::include_dir;
use log::{info, LevelFilter, Log, Metadata, Record};
use rjvm_vm::{
    io::JvmIo, java_objects_creation::extract_str_from_java_lang_string,
    value::expect_concrete_object_at, vm::Vm,
};
use unix_path::Path;
use vexide::{
    core::{
        io::{self, ErrorKind},
        sync::Mutex,
        time::Instant,
    },
    devices::screen::{Fill, Text, TextSize},
    prelude::*,
};

struct VexideLogger {
    screen: Arc<Mutex<Screen>>,
}

impl Log for VexideLogger {
    #[inline]
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        // println!("{}", record.args());
        let text = format!("{}", record.args());
        let screen = self.screen.clone();
        block_on(async move {
            const SCREEN_WIDTH_CHARS: usize = 60;
            let strings = text
                .chars()
                .collect::<Vec<_>>()
                .chunks(SCREEN_WIDTH_CHARS)
                .map(|chunk| chunk.iter().collect::<String>())
                .collect::<Vec<_>>();
            let mut screen = screen.lock().await;
            for string in strings {
                let mut text = Text::new(
                    &string,
                    TextSize::Small,
                    (0, Screen::VERTICAL_RESOLUTION - 50),
                );
                screen.scroll(0, 20);
                text.fill(&mut screen, Rgb::WHITE);
            }
            // sleep(Duration::from_millis(50)).await;
        });
    }

    fn flush(&self) {}
}

static CLASS_DIR: include_dir::Dir = include_dir!("build/classpath");

struct VexideJvmIo;

impl JvmIo for VexideJvmIo {
    fn duration_since_epoch(&self) -> core::time::Duration {
        Duration::from_secs(0)
    }

    fn is_dir(&self, path: &Path) -> bool {
        if path == Path::new("/") {
            return true;
        }
        println!("is_dir: {:?}", path);
        CLASS_DIR.get_dir(path.strip_prefix("/").unwrap()).is_some()
    }

    fn exists(&self, path: &Path) -> bool {
        if path == Path::new("/") {
            return true;
        }
        println!("exists: {:?}", path);
        CLASS_DIR
            .get_entry(path.strip_prefix("/").unwrap())
            .is_some()
    }

    fn read(&self, path: &Path) -> Result<Vec<u8>, io::Error> {
        println!("read: {:?}", path);
        let file = CLASS_DIR
            .get_file(path.strip_prefix("/").unwrap())
            .ok_or(ErrorKind::NotFound)?;
        Ok(file.contents().to_vec())
    }
}

#[vexide::main]
async fn main(peripherals: Peripherals) -> Result<(), Box<dyn Debug>> {
    let start_instant = Instant::now();
    log::set_logger(Box::leak(Box::new(VexideLogger {
        screen: Arc::new(Mutex::new(peripherals.screen)),
    })))
    .unwrap();
    log::set_max_level(LevelFilter::Info);
    info!("Making JVM");
    info!(
        "Top-level files in classpath: {:?}",
        CLASS_DIR.entries().len()
    );
    // return Ok(());
    const MEMORY_40_MB: usize = 40 * 1024 * 1024;
    let mut jvm = Vm::new(MEMORY_40_MB, Arc::new(VexideJvmIo));

    info!("Adding class path at / to JVM");
    if let Err(err) = jvm.append_class_path("/") {
        println!("error appending class path: {:?}", err);
        return Ok(());
    }

    {
        let call_stack = jvm.allocate_call_stack();
        let class_name = "org/example/App";
        let method_name = "main";
        jvm.native_methods_registry.register(
            "org/example/App",
            "vexPrintln",
            "(Ljava/lang/String;)V",
            Arc::new(|vm, _call_stack, _this, args| {
                let arg = expect_concrete_object_at(&args, 0).unwrap();
                let string_arg = extract_str_from_java_lang_string(vm, &arg).unwrap();
                info!("{string_arg}");
                Ok(None)
            }),
        );
        let main_method = match jvm.resolve_class_method(
            call_stack,
            class_name,
            method_name,
            "([Ljava/lang/String;)V",
        ) {
            Ok(method) => method,
            Err(err) => {
                println!("error resolving method: {:?}", err);
                return Ok(());
            }
        };

        info!("JVM startup time: {:?}", start_instant.elapsed());
        let main_result = jvm.invoke(call_stack, main_method, None, vec![]);
        info!("JVM total time: {:?}", start_instant.elapsed());

        // jvm.debug_stats();
        info!("{class_name}::{method_name}() = {main_result:?}");
    }
    loop {
        sleep(Duration::from_secs(1)).await;
    }
}
