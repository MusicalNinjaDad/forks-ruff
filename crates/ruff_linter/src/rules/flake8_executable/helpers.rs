#![cfg(target_family = "unix")]

use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::sync::OnceLock;

use anyhow::Result;
use tempfile::NamedTempFile;

pub(super) fn is_executable(filepath: &Path) -> Result<bool> {
    let metadata = filepath.metadata()?;
    let permissions = metadata.permissions();
    Ok(permissions.mode() & 0o111 != 0)
}

// Some file systems do not support executable bits. Instead, everything is executable.
// See #3110, #5445, #10084, #12941
//
// Benchmarking shows no noticable difference in performance if we run this check on
// all systems, as long as we use a `OnceLock`.
static EXECUTABLE_BY_DEFAULT: OnceLock<bool> = OnceLock::new();

pub(crate) fn executable_by_default(location: &Path) -> bool {
    *EXECUTABLE_BY_DEFAULT.get_or_init(|| {
        NamedTempFile::new_in(location)
            .map_err(std::convert::Into::into)
            .and_then(|tmpfile| is_executable(tmpfile.path()))
            .unwrap_or(false) // Assume a normal filesystem in case of read-only, IOError, ...
    })
}
