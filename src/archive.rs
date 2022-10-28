#[cfg(all(feature = "gzip"))]
pub mod gzip {
    use std::{fs::File, path::Path};

    use anyhow::Result;
    use flate2::read::GzDecoder;

    pub fn extract(path: &Path, destination: &Path) -> Result<()> {
        let mut archive = GzDecoder::new(File::open(&path)?);

        let mut file = File::create(&destination)?;

        std::io::copy(&mut archive, &mut file)?;

        Ok(())
    }
}

#[cfg(all(feature = "tar", feature = "gzip"))]
pub mod tar_gz {
    use std::{
        fs::{self, File},
        io,
        path::Path,
    };

    use anyhow::Result;
    use flate2::read::MultiGzDecoder;
    use tar::Archive;

    pub fn extract(path: &Path, destination: &Path) -> Result<()> {
        let mut archive = Archive::new(MultiGzDecoder::new(File::open(path)?));

        fs::create_dir(destination)?;

        for (_, file) in archive.entries().unwrap().raw(true).enumerate() {
            let mut file = file?;

            // Other file types don't work well in wasm
            let file_type = file.header().entry_type();
            if !file_type.is_dir() && !file_type.is_file() {
                continue;
            }

            let extract_path = destination.join(&file.path()?);

            if file_type.is_dir() {
                fs::create_dir_all(&extract_path)?;
            } else {
                let mut extracted_file = File::create(&extract_path)?;
                io::copy(&mut file, &mut extracted_file)?;
            }
        }
        Ok(())
    }
}

#[cfg(feature = "zip")]
pub mod zip {
    use std::{
        fs::{self, File},
        io,
        path::Path,
    };

    use anyhow::Result;
    use zip::ZipArchive;

    pub fn extract(path: &Path, destination: &Path) -> Result<()> {
        let mut archive = ZipArchive::new(File::open(path)?)?;

        fs::create_dir(destination)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;

            let safe_path = match file.enclosed_name() {
                Some(path) => path,
                None => continue,
            };

            let extract_path = destination.join(safe_path);

            if extract_path.is_dir() {
                fs::create_dir_all(&extract_path)?;
            } else {
                if let Some(parent) = extract_path.parent() {
                    if !parent.exists() {
                        fs::create_dir_all(&parent)?;
                    }
                }
                let mut extracted_file = File::create(&extract_path)?;
                io::copy(&mut file, &mut extracted_file)?;
            }
        }

        Ok(())
    }
}
