fn main() {
    // Allow `#[cfg(test_environment="wsl-ntfs")]` to be used to select specific tests for WSL
    // This is needed to validate special handling of EXE001 & EXE002
    println!("cargo::rustc-check-cfg=cfg(test_environment, values(\"wsl-ntfs\"))");
    if option_env!("RUFF_TEST_ENVIRONMENT") == Some("wsl-ntfs")  {
            println!("cargo::rustc-cfg=test_environment=\"wsl-ntfs\"");
        }
}
