use std::path::Path;

use tempfile::NamedTempFile;

use ruff_diagnostics::Diagnostic;
use ruff_python_trivia::CommentRanges;
pub(crate) use shebang_leading_whitespace::*;
pub(crate) use shebang_missing_executable_file::*;
pub(crate) use shebang_missing_python::*;
pub(crate) use shebang_not_executable::*;
pub(crate) use shebang_not_first_line::*;

use crate::codes::Rule;
use crate::comments::shebang::ShebangDirective;
use crate::rules::flake8_executable::helpers::is_executable;
use crate::settings::LinterSettings;
use crate::{warn_user_once, Locator};

mod shebang_leading_whitespace;
mod shebang_missing_executable_file;
mod shebang_missing_python;
mod shebang_not_executable;
mod shebang_not_first_line;

pub(crate) fn from_tokens(
    diagnostics: &mut Vec<Diagnostic>,
    path: &Path,
    locator: &Locator,
    comment_ranges: &CommentRanges,
    settings: &LinterSettings,
) {
    // WSL supports Windows file systems, which do not have executable bits. Instead, everything is executable.
    // Therefore, we skip EXE001 & EXE002 on WSL if we detect this situation.
    //
    // To avoid unnecessary performance penalties, and offer a workaround for #12941, the user can
    // identify the filesystem by setting the environment variable `RUFF_WSL_FILESYSTEM`
    let wsl_ntfs = 
        match std::env::var("RUFF_WSL_FILESYSTEM") {
            
            Ok(value) => value == *"ntfs",
            
            Err(_) => {
                // Only run these checks on WSL - users mounting FAT, NTFS, CIFS etc. in other cases (#12941) must set `RUFF_WSL_FILESYSTEM`.
                if is_wsl::is_wsl() {
                    // Create a tempfile in the project root and see whether it is executable by default.
                    // If we run into some kind of error with the tempfile, we'll assume people are following MS recommendation and using the WSL native filesystem
                    match NamedTempFile::new_in(settings.project_root.as_path()).map_err(std::convert::Into::into) 
                        .and_then(|tmpfile| is_executable(tmpfile.path()))
                    {
                        Ok(executable) if executable => {
                            warn_user_once!("EXE001/EXE002 is not available on WSL when a windows filesystem is mounted - see the docs for more information.");
                            true
                        }
                        _ => {
                            warn_user_once!("EXE001/EXE002 incur a small performance hit on WSL unless RUFF_WSL_FILESYSTEM is set - see the docs for more information.");
                            false
                        }
                    }
                }
                else {false}
            }
        };

    let mut has_any_shebang = false;
    for range in comment_ranges {
        let comment = locator.slice(range);
        if let Some(shebang) = ShebangDirective::try_extract(comment) {
            has_any_shebang = true;

            if let Some(diagnostic) = shebang_missing_python(range, &shebang) {
                diagnostics.push(diagnostic);
            }

            if settings.rules.enabled(Rule::ShebangNotExecutable) && !wsl_ntfs {
                if let Some(diagnostic) = shebang_not_executable(path, range) {
                    diagnostics.push(diagnostic);
                }
            }

            if let Some(diagnostic) = shebang_leading_whitespace(range, locator) {
                diagnostics.push(diagnostic);
            }

            if let Some(diagnostic) = shebang_not_first_line(range, locator) {
                diagnostics.push(diagnostic);
            }
        }
    }

    if !has_any_shebang {
        if settings.rules.enabled(Rule::ShebangMissingExecutableFile) && !wsl_ntfs {
            if let Some(diagnostic) = shebang_missing_executable_file(path) {
                diagnostics.push(diagnostic);
            }
        }
    }
}
