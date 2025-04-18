fn main() {
    // Allow `#[cfg(wsl)]` to be used to select specific tests for WSL
    // This is needed to validate special handling of EXE001 & EXE002
    println!("cargo::rustc-check-cfg=cfg(testing_on_wsl)");
    if is_wsl::is_wsl() {
        // The RUFF_WSL_FILESYSTEM environment variable overrides WSL handling
        // If set when testing: run non-wsl tests
        if option_env!("RUFF_WSL_FILESYSTEM") != Some("ext4")  {
            println!("cargo::rustc-cfg=testing_on_wsl");
        }
    }
}
