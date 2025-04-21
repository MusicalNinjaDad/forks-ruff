use std::cell::OnceCell;
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
use crate::Locator;

mod shebang_leading_whitespace;
mod shebang_missing_executable_file;
mod shebang_missing_python;
mod shebang_not_executable;
mod shebang_not_first_line;

// WSL supports Windows file systems, which do not have executable bits. Instead, everything is executable.
// Therefore, we skip EXE001 & EXE002 on WSL if we detect this situation.
//
// To avoid performance penalties, and offer a workaround for #12941, the user can
// identify the filesystem by setting the environment variable `RUFF_WSL_FILESYSTEM`
fn wsl_ntfs_check(project_root: &Path) -> bool {
    // ?? Is a OnceCell OK here - or do we need a thread-safe static alternative ??
    let wsl_ntfs: OnceCell<bool> = OnceCell::new();

    *wsl_ntfs.get_or_init(|| {
        NamedTempFile::new_in(project_root).map_err(std::convert::Into::into)
            .and_then(|tmpfile| is_executable(tmpfile.path()))
            .unwrap_or(false)
    })
}

pub(crate) fn from_tokens(
    diagnostics: &mut Vec<Diagnostic>,
    path: &Path,
    locator: &Locator,
    comment_ranges: &CommentRanges,
    settings: &LinterSettings,
) {
    let mut has_any_shebang = false;
    for range in comment_ranges {
        let comment = locator.slice(range);
        if let Some(shebang) = ShebangDirective::try_extract(comment) {
            has_any_shebang = true;

            if let Some(diagnostic) = shebang_missing_python(range, &shebang) {
                diagnostics.push(diagnostic);
            }

            if settings.rules.enabled(Rule::ShebangNotExecutable)
                && !wsl_ntfs_check(&settings.project_root)
            {
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
        if settings.rules.enabled(Rule::ShebangMissingExecutableFile)
            && !wsl_ntfs_check(&settings.project_root)
        {
            if let Some(diagnostic) = shebang_missing_executable_file(path) {
                diagnostics.push(diagnostic);
            }
        }
    }
}
