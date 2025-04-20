fn main() {
    // Allow e.g. `#[cfg(test_environment="wsl:ntfs")]` to be used to select specific tests for WSL
    // This is needed to validate special handling of EXE001 & EXE002
    // Format is a list of values, separated using the OS-specific path separator (`:` on WSL/*nix, ";" on Windows)
    println!("cargo::rustc-check-cfg=cfg(test_environment, values(\"wsl\",\"ntfs\",\"ruff_wsl_filesystem_is_set\"))");
    if let Some(list_of_values) = option_env!("RUFF_TEST_ENVIRONMENT") {
            for value in std::env::split_paths(list_of_values) {
                println!("cargo::rustc-cfg=test_environment=\"{}\"", value.to_string_lossy());}
        }
}
