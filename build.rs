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
