use cmake::Config;
use std::{env, path::PathBuf};

fn main() {
    #[cfg(feature = "build-nng")]
    {
        let generator = generator();
        let stats = if cfg!(feature = "nng-stats") { "ON" } else { "OFF" };
        let tls = if cfg!(feature = "nng-tls") { "ON" } else { "OFF" };

        let dst = Config::new("nng")
            .generator(generator)
            .define("NNG_TESTS", "OFF")
            .define("NNG_TOOLS", "OFF")
            .define("NNG_ENABLE_STATS", stats)
            .define("NNG_ENABLE_TLS", tls)
            .build();

        println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
        println!("cargo:rustc-link-search=native={}", dst.join("lib64").display());
        println!("cargo:rustc-link-lib=static=nng");
    }
    #[cfg(not(feature = "build-nng"))]
    {
        println!("cargo:rustc-link-lib=dylib=nng");
    }


    // https://rust-lang-nursery.github.io/rust-bindgen
    // https://docs.rs/bindgen
    let mut builder = bindgen::Builder::default()
        // This is needed if use `#include <nng.h>` instead of `#include "path/nng.h"` in wrapper.h
        //.clang_arg("-Inng/src/")
        .header("wrapper.h");

    builder = builder
        .whitelist_type("nng_.*")
        .whitelist_function("nng_.*")
        .whitelist_var("NNG_.*")
        // Generate `pub const NNG_UNIT_EVENTS` instead of `nng_unit_enum_NNG_UNIT_EVENTS`
        .prepend_enum_name(false)
        // Generate `pub enum ...` instead of multiple `pub const ...`
        .rustified_enum("nng_.*_enum")
        // Enum special cases:
        .rustified_enum("nng_pipe_ev")
        .rustified_enum("nng_sockaddr_family")
        .rustified_enum("nng_zt_status")
        // no_std support
        // https://rust-embedded.github.io/book/interoperability/c-with-rust.html#automatically-generating-the-interface
        .ctypes_prefix("cty")
        .use_core();
    let bindings = builder.generate().expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}

fn generator() -> &'static str {
    let ninja = cfg!(feature = "cmake-ninja");
    let vs2017 = cfg!(feature = "cmake-vs2017");
    let vs2017_win64 = cfg!(feature = "cmake-vs2017-win64");
    let unix = cfg!(target_family = "unix");

    match (ninja, vs2017, vs2017_win64, unix) {
        (true, false, false, _) => "Ninja",
        (false, true, false, _) => "Visual Studio 15 2017",
        (false, false, true, _) => "Visual Studio 15 2017 Win64",
        (false, false, false, true) => "Unix Makefiles",
        (false, false, false, false) => "Ninja",
        _ => panic!("More than one CMake generator selected"),
    }
}
