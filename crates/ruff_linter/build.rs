fn main() {
    // Allow `#[cfg(wsl)]` to be used to select specific tests for WSL
    // This is needed to validate special handling of EXE001 & EXE002
    println!("cargo::rustc-check-cfg=cfg(wsl)");
    
    // Set the `wsl` cfg flag for tests (only)
    #[cfg(test)]
    if is_wsl::is_wsl() {
        println!("cargo::rustc-cfg=wsl");
    }
}
