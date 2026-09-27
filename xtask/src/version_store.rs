use camino::Utf8Path;
use rootcause::Report;

type Result<T> = std::result::Result<T, Report>;

const VERSION_FILE: &str = "version.json";

#[derive(Debug, Clone, facet::Facet)]
#[facet(rename_all = "snake_case")]
struct VersionEntry {
    pub version: String,
}

pub fn write_version(repo_root: &Utf8Path, version: &str) -> Result<()> {
    let path = repo_root.join(VERSION_FILE);
    let entry = VersionEntry {
        version: version.to_string(),
    };
    let content = facet_json::to_string_pretty(&entry).expect("version entry serialization failed");
    fs_err::write(&path, content)?;

    // Also update packages/arborium/package.json
    sync_main_npm_package_version(repo_root, version)?;

    Ok(())
}

/// Ensure packages/arborium/package.json matches the canonical version.
pub fn sync_main_npm_package_version(repo_root: &Utf8Path, version: &str) -> Result<()> {
    update_main_npm_package_version(repo_root, version)
}

/// Update the version in packages/arborium/package.json
fn update_main_npm_package_version(repo_root: &Utf8Path, version: &str) -> Result<()> {
    let package_json_path = repo_root.join("packages/arborium/package.json");

    if !package_json_path.exists() {
        // Package will be generated from template by `xtask gen`
        // The template will use the version from version.json
        return Ok(());
    }

    let content = fs_err::read_to_string(&package_json_path)?;

    // Parse as serde_json::Value to preserve structure
    let mut json: serde_json::Value = serde_json::from_str(&content)?;

    // Update version field
    if let Some(obj) = json.as_object_mut() {
        obj.insert(
            "version".to_string(),
            serde_json::Value::String(version.to_string()),
        );
    }

    // Write back with pretty formatting
    let updated = serde_json::to_string_pretty(&json)?;

    fs_err::write(&package_json_path, updated + "\n")?;

    Ok(())
}

/// Default version for local development.
/// CI should always pass --version from the git tag.
pub const DEV_VERSION: &str = "0.0.0";

pub fn read_version(repo_root: &Utf8Path) -> Result<String> {
    let path = repo_root.join(VERSION_FILE);
    match fs_err::read_to_string(&path) {
        Ok(content) => {
            let entry: VersionEntry = facet_json::from_str(&content)?;
            Ok(entry.version)
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            // Fresh Git checkouts use the committed package version. CI can still
            // override it through version.json or `gen --version`.
            let manifest = repo_root.join("crates/arborium/Cargo.toml");
            match fs_err::read_to_string(manifest) {
                Ok(content) => {
                    let manifest: toml::Value = toml::from_str(&content)?;
                    let version = manifest
                        .get("package")
                        .and_then(|package| package.get("version"))
                        .and_then(toml::Value::as_str)
                        .ok_or_else(|| rootcause::report!("arborium package version is missing"))?;
                    Ok(version.to_string())
                }
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                    Ok(DEV_VERSION.to_string())
                }
                Err(err) => Err(err.into()),
            }
        }
        Err(err) => Err(err.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fresh_checkout_uses_committed_version_with_release_override() {
        let temp = tempfile::tempdir().unwrap();
        let root = Utf8Path::from_path(temp.path()).unwrap();
        fs_err::create_dir_all(root.join("crates/arborium")).unwrap();
        fs_err::write(
            root.join("crates/arborium/Cargo.toml"),
            "[package]\nversion = \"2.18.2\"\n",
        )
        .unwrap();
        assert_eq!(read_version(root).unwrap(), "2.18.2");

        write_version(root, "3.0.0").unwrap();
        assert_eq!(read_version(root).unwrap(), "3.0.0");
    }
}
