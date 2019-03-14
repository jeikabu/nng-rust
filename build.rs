fn main() {
    link_nng();
    build_bindgen();
}

#[cfg(feature = "build-nng")]
fn link_nng() {
    struct Generator(&'static str);
    const UNIX_MAKEFILES: Generator = Generator("Unix Makefiles");
    const NINJA: Generator = Generator("Ninja");
    const VS2017_WIN64: Generator = Generator("Visual Studio 15 2017 Win64");
    const VS2017: Generator = Generator("Visual Studio 15 2017");

    // Compile time settings
    let generator = if cfg!(feature = "cmake-unix") {
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
    };

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
    let dst = cmake::Config::new("nng")
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
}

#[cfg(not(feature = "build-nng"))]
fn link_nng() {
    println!("cargo:rustc-link-lib=dylib=nng");
}

#[cfg(feature = "build-bindgen")]
fn build_bindgen() {
    use std::{env, path::PathBuf};

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
        .use_core()
        .parse_callbacks(Box::new(BindgenCallbacks::default()))
        // Layout tests are non-portable; 64-bit tests are "wrong" size on 32-bit and always fail.
        // Don't output tests if we're regenerating `src/bindings.rs` (shared by all platforms when bindgen not used)
        .layout_tests(!cfg!(feature = "source-update-bindings"));

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

    const BINDINGS_RS: &str = "bindings.rs";
    let out_file = PathBuf::from(env::var("OUT_DIR").unwrap()).join(BINDINGS_RS);
    builder
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_file.to_owned())
        .expect("Couldn't write bindings");

    #[cfg(feature = "source-update-bindings")]
    {
        let bindings = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("src")
            .join(BINDINGS_RS);
        std::fs::copy(out_file, bindings).expect("Unable to update bindings");
    }
}

#[cfg(not(feature = "build-bindgen"))]
fn build_bindgen() {
    // Nothing
}

#[derive(Debug, Default)]
struct BindgenCallbacks;

#[cfg(feature = "build-bindgen")]
impl bindgen::callbacks::ParseCallbacks for BindgenCallbacks {
    fn enum_variant_behavior(
        &self,
        _enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<bindgen::callbacks::EnumVariantCustomBehavior> {
        // nng_pipe_ev::NNG_PIPE_EV_NUM is only used in NNG internals to validate range of values.
        // We want to exclude it so it doesn't need to be included for `match` to be exhaustive.
        if original_variant_name == "NNG_PIPE_EV_NUM" {
            Some(bindgen::callbacks::EnumVariantCustomBehavior::Hide)
        } else {
            None
        }
    }
}
