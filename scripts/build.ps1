# Non-terminating errors fail the script immediately
$ErrorActionPreference = "Stop"

if ($IsMacOS) {
    $env:PATH += [IO.Path]::PathSeparator + "$env:HOME/.cargo/bin"
}

cargo fmt --all -- --check
if ($IsWindows) {
    # Note: currently only VS2019 has clang/llvm installed.  So, VS2017 can't run bindgen.
    # https://github.com/microsoft/azure-pipelines-image-generation/pull/1297

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
        # For 64-bit builds auto-detect correct generator
        if ($env:vs_ver -eq "2019") {
            # Have clang/llvm, so can run bindgen
            $env:LIBCLANG_PATH = "$env:VCINSTALLDIR/Tools/Llvm/bin"
            cargo test --features build-bindgen
            cargo build --features source-update-bindings
        }
        else {
            # No clang/llvm, so can't run bindgen
            cargo test
        }
    }
} elseif ($IsMacOS) {
    cargo test --features build-bindgen
    cargo build --features source-update-bindings
} else {
    cargo test
}