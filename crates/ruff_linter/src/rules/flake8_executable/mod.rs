//! Rules from [flake8-executable](https://pypi.org/project/flake8-executable/).
pub(crate) mod helpers;
pub(crate) mod rules;

#[cfg(unix)]
#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use crate::test::{test_path, test_resource_path};
    use crate::{
        assert_messages, registry, settings::rule_table::RuleTable, settings::LinterSettings,
        RuleSelector,
    };
    use anyhow::Result;
    use test_case::test_case;

    fn has_pyproject_toml(pyproject_toml: bool) -> PathBuf {
        let location = if pyproject_toml {
            test_resource_path("fixtures").join("flake8_executable")
        } else {
            test_resource_path("fixtures")
        };
        assert_eq!(location.join("pyproject.toml").exists(), pyproject_toml, "Error setting up a project_root with(out) pyproject.toml");
        location
    }

    #[cfg_attr(not(test_environment = "ntfs"), test_case(Path::new("EXE001_1.py")))]
    #[cfg_attr(test_environment = "ntfs", test_case(Path::new("EXE001_1_wsl.py")))]
    #[test_case(Path::new("EXE001_2.py"))]
    #[test_case(Path::new("EXE001_3.py"))]
    #[cfg_attr(not(test_environment = "ntfs"), test_case(Path::new("EXE002_1.py")))]
    #[cfg_attr(test_environment = "ntfs", test_case(Path::new("EXE002_1_wsl.py")))]
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
    fn rules_with_pyproject_toml(path: &Path) -> Result<()> {
        let snapshot = path.to_string_lossy().into_owned();
        let rules: RuleTable = RuleSelector::Linter(registry::Linter::Flake8Executable)
            .all_rules()
            .collect();

        let settings = LinterSettings {
            rules,
            project_root: has_pyproject_toml(true),
            ..LinterSettings::default()
        };

        let diagnostics = test_path(
            Path::new("flake8_executable").join(path).as_path(),
            &settings,
        )?;

        assert_messages!(snapshot, diagnostics);
        Ok(())
    }

    #[cfg_attr(not(test_environment = "ntfs"), test_case(Path::new("EXE001_1.py")))]
    #[cfg_attr(test_environment = "ntfs", test_case(Path::new("EXE001_1_wsl.py")))]
    #[test_case(Path::new("EXE001_2.py"))]
    #[test_case(Path::new("EXE001_3.py"))]
    #[cfg_attr(not(test_environment = "ntfs"), test_case(Path::new("EXE002_1.py")))]
    #[cfg_attr(test_environment = "ntfs", test_case(Path::new("EXE002_1_wsl.py")))]
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
    fn rules_without_pyproject_toml(path: &Path) -> Result<()> {
        let snapshot = path.to_string_lossy().into_owned();
        let rules: RuleTable = RuleSelector::Linter(registry::Linter::Flake8Executable)
            .all_rules()
            .collect();

        let settings = LinterSettings {
            rules,
            project_root: has_pyproject_toml(false),
            ..LinterSettings::default()
        };

        let diagnostics = test_path(
            Path::new("flake8_executable").join(path).as_path(),
            &settings,
        )?;

        assert_messages!(snapshot, diagnostics);
        Ok(())
    }
}
