
# Check if Rust already installed
$have_rustup = $false
try {
    if (Get-Command rustup) {
        $have_rustup = $true
    }
} catch {
    # Do nothing
}

if ($have_rustup) {
    rustup update
} else {
    # Install Rust
    if ($IsWindows) {
        Invoke-WebRequest https://win.rustup.rs/ -OutFile rustup-init.exe
        ./rustup-init.exe -yv --default-toolchain stable --default-host x86_64-pc-windows-msvc
    } elseif ($IsMacOS) {
        Invoke-WebRequest https://sh.rustup.rs -OutFile rustup-init.sh
        bash rustup-init.sh -y -v --default-toolchain stable
    } else {
    
    }
}
