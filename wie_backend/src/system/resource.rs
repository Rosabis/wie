use crate::extract_zip;

pub struct Resource {
    files: Vec<(String, Vec<u8>)>,
}

impl Default for Resource {
    fn default() -> Self {
        Self::new()
    }
}

impl Resource {
    pub fn new() -> Self {
        Self { files: Vec::new() }
    }

    pub fn add(&mut self, path: &str, data: Vec<u8>) {
        tracing::trace!("Adding resource {}, {}b", path, data.len());

        self.files.push((path.to_string(), data));
    }

    pub fn id(&self, path: &str) -> Option<u32> {
        tracing::trace!("Looking for resource {}", path);

        if let Some(x) = path.strip_prefix('/') {
            return self.id(x);
        }

        for (id, file) in self.files.iter().enumerate() {
            if file.0 == path {
                return Some(id as _);
            }
        }

        tracing::warn!("No such resource {}", path);

        None
    }

    pub fn size(&self, id: u32) -> u32 {
        self.files[id as usize].1.len() as _
    }

    pub fn data(&self, id: u32) -> &[u8] {
        &self.files[id as usize].1
    }

    pub fn files(&self) -> impl Iterator<Item = &str> {
        self.files.iter().map(|file| file.0.as_ref())
    }

    pub fn mount_zip(&mut self, zip: &[u8]) -> anyhow::Result<()> {
        let files = extract_zip(zip)?;

        for (path, data) in files {
            self.add(&path, data);
        }

        Ok(())
    }
}
