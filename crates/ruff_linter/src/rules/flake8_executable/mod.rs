//! Rules from [flake8-executable](https://pypi.org/project/flake8-executable/).
pub(crate) mod helpers;
pub(crate) mod rules;

#[cfg(unix)]
#[cfg(test)]
mod tests {
    use std::path::Path;

    use anyhow::Result;
    #[cfg(test_environment="ntfs")]
    use log::Level;
    use test_case::test_case;

    use crate::registry::Rule;
    use crate::test::test_path;
    use crate::{assert_messages, settings};

    #[test_case(Path::new("EXE001_1.py"))]
    #[test_case(Path::new("EXE001_2.py"))]
    #[test_case(Path::new("EXE001_3.py"))]
    #[test_case(Path::new("EXE002_1.py"))]
    #[test_case(Path::new("EXE002_2.py"))]
    #[test_case(Path::new("EXE002_3.py"))]
    #[test_case(Path::new("EXE003.py"))]
    #[test_case(Path::new("EXE003_uv.py"))]
    #[test_case(Path::new("EXE004_1.py"))]
    #[test_case(Path::new("EXE004_2.py"))]
    #[test_case(Path::new("EXE004_3.py"))]
    #[test_case(Path::new("EXE004_4.py"))]
    #[test_case(Path::new("EXE005_1.py"))]
    #[test_case(Path::new("EXE005_2.py"))]
    #[test_case(Path::new("EXE005_3.py"))]
    #[cfg(not(test_environment="ntfs"))]
    fn rules(path: &Path) -> Result<()> {
        let snapshot = path.to_string_lossy().into_owned();
        let diagnostics = test_path(
            Path::new("flake8_executable").join(path).as_path(),
            &settings::LinterSettings::for_rules(vec![
                Rule::ShebangNotExecutable,
                Rule::ShebangMissingExecutableFile,
                Rule::ShebangLeadingWhitespace,
                Rule::ShebangNotFirstLine,
                Rule::ShebangMissingPython,
            ]),
        )?;
        assert_messages!(snapshot, diagnostics);
        Ok(())
    }

    #[test_case(Path::new("EXE001_1.py"))]
    #[test_case(Path::new("EXE002_1.py"))]
    #[cfg(
        any(
            not(test_environment="wsl"), // OR
            test_environment="ruff_wsl_filesystem_is_set"
        )
    )]
    fn warnings(path: &Path) -> Result<()> {
        testing_logger::setup();
        test_path(
            Path::new("flake8_executable").join(path).as_path(),
            &settings::LinterSettings::for_rules(vec![
                Rule::ShebangNotExecutable,
                Rule::ShebangMissingExecutableFile
            ]),
        )?;
        testing_logger::validate( |captured_logs| {
            assert_eq!(captured_logs.len(), 0);
        });
        Ok(())
    }

    #[test_case(Path::new("EXE001_1.py"))]
    #[test_case(Path::new("EXE002_1.py"))]
    #[cfg(
        all(
            test_environment="ntfs", // AND
            not(test_environment="ruff_wsl_filesystem_is_set")
        )
    )]
    fn warnings(path: &Path) -> Result<()> {
        testing_logger::setup();
        test_path(
            Path::new("flake8_executable").join(path).as_path(),
            &settings::LinterSettings::for_rules(vec![
                Rule::ShebangNotExecutable,
                Rule::ShebangMissingExecutableFile
            ]),
        )?;
        testing_logger::validate( |captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].body, "EXE001/EXE002 is not available on WSL when a windows filesystem is mounted - see the docs for more information.");
            assert_eq!(captured_logs[0].level, Level::Warn);
        });
        Ok(())
    }

    #[test_case(Path::new("EXE001_1.py"))]
    #[test_case(Path::new("EXE002_1.py"))]
    #[cfg(
        all(
            test_environment="wsl", // AND
            not(test_environment="ntfs"), // AND
            not(test_environment="ruff_wsl_filesystem_is_set")
        )
    )]
    fn warnings(path: &Path) -> Result<()> {
        testing_logger::setup();
        test_path(
            Path::new("flake8_executable").join(path).as_path(),
            &settings::LinterSettings::for_rules(vec![
                Rule::ShebangNotExecutable,
                Rule::ShebangMissingExecutableFile
            ]),
        )?;
        testing_logger::validate( |captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].body, "EXE001/EXE002 incur a small performance hit on WSL unless RUFF_WSL_FILESYSTEM is set - see the docs for more information.");
            assert_eq!(captured_logs[0].level, Level::Warn);
        });
        Ok(())
    }

    #[test_case(Path::new("EXE001_1_wsl.py"))]
    #[test_case(Path::new("EXE001_2.py"))]
    #[test_case(Path::new("EXE001_3.py"))]
    #[test_case(Path::new("EXE002_1_wsl.py"))]
    #[test_case(Path::new("EXE002_2.py"))]
    #[test_case(Path::new("EXE002_3.py"))]
    #[test_case(Path::new("EXE003.py"))]
    #[test_case(Path::new("EXE003_uv.py"))]
    #[test_case(Path::new("EXE004_1.py"))]
    #[test_case(Path::new("EXE004_2.py"))]
    #[test_case(Path::new("EXE004_3.py"))]
    #[test_case(Path::new("EXE004_4.py"))]
    #[test_case(Path::new("EXE005_1.py"))]
    #[test_case(Path::new("EXE005_2.py"))]
    #[test_case(Path::new("EXE005_3.py"))]
    #[cfg(test_environment="ntfs")]
    fn rules_wsl(path: &Path) -> Result<()> {
        let snapshot = path.to_string_lossy().into_owned();
        let diagnostics = test_path(
            Path::new("flake8_executable").join(path).as_path(),
            &settings::LinterSettings::for_rules(vec![
                Rule::ShebangNotExecutable,
                Rule::ShebangMissingExecutableFile,
                Rule::ShebangLeadingWhitespace,
                Rule::ShebangNotFirstLine,
                Rule::ShebangMissingPython,
            ]),
        )?;
        assert_messages!(snapshot, diagnostics);
        Ok(())
    }
}
