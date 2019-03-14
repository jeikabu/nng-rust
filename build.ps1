# Non-terminating errors fail the script immediately
$ErrorActionPreference = "Stop"

if ($env:platform -eq "x86") {
    cargo test --features cmake-vs2017
} else {
    cargo test --features build-bindgen
    cargo build --features source-update-bindings
}