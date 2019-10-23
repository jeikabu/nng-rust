# Non-terminating errors fail the script immediately
$ErrorActionPreference = "Stop"

if ($IsMacOS) {
    $env:PATH += [IO.Path]::PathSeparator + "$env:HOME/.cargo/bin"
}

cargo fmt --all -- --check
if ($IsWindows) {
    if ($env:platform -eq "x86") {
        # For 32-bit builds explicitly set the feature to use the correct Visual Studio generator
        if ($env:vs_ver -eq "2017") {
            cargo test --features cmake-vs2017
        } elseif ($env:vs_ver -eq "2019") {
            cargo test --features cmake-vs2019
        }
        else {
            cargo test
        }
    } else {
        # For 64-bit builds auto-detect correct generator but rebuild bindings from source
        cargo test --features build-bindgen
        cargo build --features source-update-bindings
    }
} elseif ($IsMacOS) {
    cargo test --features build-bindgen
    cargo build --features source-update-bindings
} else {
    cargo test
}