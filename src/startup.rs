use include_assets::{include_dir, NamedArchive};
use std::{
    fs,
    io::{Error, ErrorKind},
    path::Path,
};
use tempdir::TempDir;
use tera::Tera;

pub struct AssetExtractor;

impl AssetExtractor {
    pub fn extract_assets() -> Result<TempDir, Error> {
        let tempdir = TempDir::new(env!("CARGO_PKG_NAME"))?;
        let archive = NamedArchive::load(include_dir!(
            "assets",
            compression = "zstd",
            level = 22,
            links = "forbid"
        ));

        for (file_name, bytes) in archive.assets() {
            let file_path = Path::new(file_name);
            Self::create_file(tempdir.path(), file_path, bytes)?;
        }

        Ok(tempdir)
    }

    fn create_file(in_dir: &Path, file_path: &Path, content: &[u8]) -> Result<(), Error> {
        let mut ancestors = file_path.ancestors();

        let target_file = in_dir.join(ancestors.next().unwrap());
        if let Some(dirs) = ancestors.next() {
            fs::create_dir_all(in_dir.join(dirs))?;
        }

        fs::write(target_file, content)
    }
}

pub fn setup_templates(asset_dir: &Path) -> Result<Tera, Error> {
    let templates_dir = asset_dir.join("templates");
    let templates_dir_str = templates_dir.to_str().unwrap();

    let tera = Tera::new(&format!("{templates_dir_str}/**"));

    tera.map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
}
