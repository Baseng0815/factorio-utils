use std::path::{Path, PathBuf};

use prototypes::IconRef;

pub trait IconResolver {
    fn resolve(&self, icon: &IconRef) -> Option<PathBuf>;
}

pub struct FactorioInstall {
    root: PathBuf,
}

impl FactorioInstall {
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self {
            root: root.as_ref().to_owned(),
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }
}

impl IconResolver for FactorioInstall {
    fn resolve(&self, icon: &IconRef) -> Option<PathBuf> {
        let (prefix, path) = split_prefix(&icon.path)?;
        let prefix_dir = match prefix {
            "base" => self.root.join("data").join("base"),
            "core" => self.root.join("data").join("core"),
            mod_name => self.root.join("mods").join(mod_name),
        };
        Some(prefix_dir.join(path))
    }
}

fn split_prefix(s: &str) -> Option<(&str, &str)> {
    let rest = s.strip_prefix("__")?;
    let end = rest.find("__")?;
    let name = &rest[..end];
    let path = rest[end + 2..].strip_prefix('/').unwrap_or(&rest[end + 2..]);
    Some((name, path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_base_prefix() {
        let (name, path) = split_prefix("__base__/graphics/icons/iron-plate.png").unwrap();
        assert_eq!(name, "base");
        assert_eq!(path, "graphics/icons/iron-plate.png");
    }

    #[test]
    fn splits_mod_prefix_with_hyphens() {
        let (name, path) = split_prefix("__some-mod-name__/icon.png").unwrap();
        assert_eq!(name, "some-mod-name");
        assert_eq!(path, "icon.png");
    }

    #[test]
    fn rejects_no_prefix() {
        assert!(split_prefix("/just/a/path.png").is_none());
        assert!(split_prefix("__base/missing-end.png").is_none());
    }

    #[test]
    fn factorio_install_resolves_base() {
        let install = FactorioInstall::new("/opt/factorio");
        let icon = IconRef::new("__base__/graphics/icons/iron-plate.png", 64);
        let resolved = install.resolve(&icon).unwrap();
        assert_eq!(
            resolved,
            PathBuf::from("/opt/factorio/data/base/graphics/icons/iron-plate.png"),
        );
    }

    #[test]
    fn factorio_install_resolves_core() {
        let install = FactorioInstall::new("/opt/factorio");
        let icon = IconRef::new("__core__/graphics/icons/foo.png", 64);
        let resolved = install.resolve(&icon).unwrap();
        assert_eq!(
            resolved,
            PathBuf::from("/opt/factorio/data/core/graphics/icons/foo.png"),
        );
    }

    #[test]
    fn factorio_install_resolves_mod() {
        let install = FactorioInstall::new("/opt/factorio");
        let icon = IconRef::new("__bobs-mods__/graphics/icons/bob.png", 64);
        let resolved = install.resolve(&icon).unwrap();
        assert_eq!(
            resolved,
            PathBuf::from("/opt/factorio/mods/bobs-mods/graphics/icons/bob.png"),
        );
    }
}
