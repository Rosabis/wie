use alloc::{boxed::Box, string::String, vec::Vec};

use wie_backend::{App, Archive, Backend};

use crate::app::J2MEApp;

pub struct J2MEArchive {
    jar: Vec<u8>,
}

impl J2MEArchive {
    pub fn from_jar(data: Vec<u8>) -> Self {
        Self { jar: data } // TODO get main class from manifest
    }
}

impl Archive for J2MEArchive {
    fn id(&self) -> String {
        todo!()
    }

    fn load_app(&self, backend: &mut Backend) -> anyhow::Result<Box<dyn App>> {
        backend.mount_zip(&self.jar)?;

        Ok(Box::new(J2MEApp::new("", backend)?))
    }
}