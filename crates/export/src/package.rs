use crate::{ExportError as E, Result};
use serde::Serialize;
use std::{collections::BTreeMap, io::Write, path::Path};

struct Bounded {
    bytes: Vec<u8>,
    limit: usize,
}
impl Write for Bounded {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if bytes.len() > self.limit.saturating_sub(self.bytes.len()) {
            return Err(std::io::ErrorKind::FileTooLarge.into());
        }
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
pub(crate) fn bounded_json(value: &impl Serialize, limit: usize) -> Result<Vec<u8>> {
    let mut writer = Bounded {
        bytes: vec![],
        limit,
    };
    serde_json::to_writer_pretty(&mut writer, value).map_err(|_| E::OutputLimit)?;
    Ok(writer.bytes)
}
pub(crate) fn bounded_compact_json(value: &impl Serialize, limit: usize) -> Result<String> {
    let mut writer = Bounded {
        bytes: vec![],
        limit,
    };
    serde_json::to_writer(&mut writer, value).map_err(|_| E::OutputLimit)?;
    String::from_utf8(writer.bytes).map_err(|_| E::InvalidInput)
}
/// Fully validated immutable contents. Construction is through `compile` only.
#[derive(Debug)]
pub struct Package {
    pub(crate) files: BTreeMap<String, Vec<u8>>,
    pub(crate) comparison_attribution: &'static str,
}
impl Package {
    pub fn comparison_attribution(&self) -> &'static str {
        self.comparison_attribution
    }
    pub fn files(&self) -> &BTreeMap<String, Vec<u8>> {
        &self.files
    }
    /// Explicit destination only; existing directories, baselines and symlinks are refused.
    /// On IO failure, removes only the new directory/files this call created.
    pub fn write_new(&self, destination: &Path) -> Result<()> {
        std::fs::create_dir(destination).map_err(|e| {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                E::DestinationExists
            } else {
                E::Io
            }
        })?;
        let mut created = vec![];
        let result = (|| {
            for (name, bytes) in &self.files {
                let path = destination.join(name);
                let mut file = std::fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&path)
                    .map_err(|_| E::Io)?;
                created.push(path);
                file.write_all(bytes)
                    .and_then(|()| file.sync_all())
                    .map_err(|_| E::Io)?;
            }
            Ok(())
        })();
        if result.is_err() {
            for path in created {
                let _ = std::fs::remove_file(path);
            }
            let _ = std::fs::remove_dir(destination);
        }
        result
    }
}
