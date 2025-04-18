fn main() {
    // Allow cfg(wsl) to be used to select specific tests for WSL
    // This is needed to validate special handling of EXE001 & EXE002
    // Not calculated - to avoid dev dependency on is_wsl
    // Use `RUSTFLAGS="--cfg wsl" cargo insta ...`
    println!("cargo::rustc-check-cfg=cfg(wsl)");
}