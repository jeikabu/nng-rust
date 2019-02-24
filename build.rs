use cmake::Config;
use std::{env, path::PathBuf};

fn main() {
    if cfg!(feature = "build-nng") {
        let generator = generator();
        let stats = if cfg!(feature = "nng-stats") {
            "ON"
        } else {
            "OFF"
        };
        let tls = if cfg!(feature = "nng-tls") {
            "ON"
        } else {
            "OFF"
        };

        // Run cmake to build nng
        let dst = Config::new("nng")
            .generator(generator.0)
            .define("NNG_TESTS", "OFF")
            .define("NNG_TOOLS", "OFF")
            .define("NNG_ENABLE_STATS", stats)
            .define("NNG_ENABLE_TLS", tls)
            .build();

        // Check output of `cargo build --verbose`, should see something like:
        // -L native=/path/runng/target/debug/build/runng-sys-abc1234/out
        // That contains output from cmake
        println!(
            "cargo:rustc-link-search=native={}",
            dst.join("lib").display()
        );
        println!(
            "cargo:rustc-link-search=native={}",
            dst.join("lib64").display()
        );

        println!("cargo:rustc-link-lib=static=nng");
    } else {
        println!("cargo:rustc-link-lib=dylib=nng");
    }

    if cfg!(feature = "build-bindgen") {
        let mut builder = bindgen::Builder::default()
            // This is needed if use `#include <nng.h>` instead of `#include "path/nng.h"` in wrapper.h
            //.clang_arg("-Inng/src/")
            .header("src/wrapper.h")
            // #[derive(Default)]
            .derive_default(true)
            .whitelist_type("nng_.*")
            .whitelist_function("nng_.*")
            .whitelist_var("NNG_.*")
            .opaque_type("nng_.*_s")
            // Generate `pub const NNG_UNIT_EVENTS` instead of `nng_unit_enum_NNG_UNIT_EVENTS`
            .prepend_enum_name(false)
            // Generate `pub enum ...` instead of multiple `pub const ...`
            .default_enum_style(bindgen::EnumVariation::Rust)
            .constified_enum("nng_flag_enum")
            // NNG_ESYSERR and NNG_ETRANERR are used like flag
            .constified_enum("nng_errno_enum")
            .use_core();

        if cfg!(feature = "nng-compat") {
            builder = builder.header("compat.h");
        }
        if cfg!(feature = "nng-supplemental") {
            builder = builder.header("supplemental.h");
        }
        if cfg!(feature = "no_std") {
            // no_std support
            // https://rust-embedded.github.io/book/interoperability/c-with-rust.html#automatically-generating-the-interface
            builder = builder.ctypes_prefix("cty")
        }

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        builder
            .generate()
            .expect("Unable to generate bindings")
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings");
    }
}

struct Generator(&'static str);

fn generator() -> Generator {
    const UNIX_MAKEFILES: Generator = Generator("Unix Makefiles");
    const NINJA: Generator = Generator("Ninja");
    const VS2017_WIN64: Generator = Generator("Visual Studio 15 2017 Win64");
    const VS2017: Generator = Generator("Visual Studio 15 2017");

    // Compile-time features
    if cfg!(feature = "cmake-unix") {
        UNIX_MAKEFILES
    } else if cfg!(feature = "cmake-ninja") {
        NINJA
    } else if cfg!(feature = "cmake-vs2017-win64") {
        VS2017_WIN64
    } else if cfg!(feature = "cmake-vs2017") {
        VS2017
    } else {
        // Default generators
        if cfg!(target_family = "unix") {
            UNIX_MAKEFILES
        } else {
            VS2017_WIN64
        }
    }
}
