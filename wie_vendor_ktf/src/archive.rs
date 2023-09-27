use alloc::{boxed::Box, format, string::String, vec::Vec};

use anyhow::Context;

use wie_backend::{extract_zip, App, Archive, Backend};

use crate::app::KtfWipiApp;

pub struct KtfArchive {
    jar: Vec<u8>,
    main_class_name: String,
}

impl KtfArchive {
    pub fn from_zip(data: &[u8]) -> anyhow::Result<Self> {
        let mut files = extract_zip(data)?;

        let adf = files.get("__adf__").context("Invalid format")?;
        let adf = KtfAdf::parse(adf);

        tracing::info!("Loading app {}, mclass {}", adf.aid, adf.mclass);

        let jar = files.remove(&format!("{}.jar", adf.aid)).context("Invalid format")?;

        // TODO load resource on P directory

        Ok(Self::from_jar(jar, &adf.mclass))
    }

    pub fn from_jar(data: Vec<u8>, main_class_name: &str) -> Self {
        Self {
            jar: data,
            main_class_name: main_class_name.into(),
        }
    }
}

impl Archive for KtfArchive {
    fn load_app(&self, backend: &mut Backend) -> anyhow::Result<Box<dyn App>> {
        let mut jar_data = extract_zip(&self.jar)?;

        for (filename, data) in jar_data.drain() {
            backend.add_resource(&filename, data);
        }

        Ok(Box::new(KtfWipiApp::new(&self.main_class_name, backend)?))
    }
}

struct KtfAdf {
    aid: String,
    mclass: String,
}

impl KtfAdf {
    pub fn parse(data: &[u8]) -> Self {
        let mut aid = String::new();
        let mut mclass = String::new();

        let mut lines = data.split(|x| *x == b'\n');

        for line in &mut lines {
            if line.starts_with(b"AID:") {
                aid = String::from_utf8_lossy(&line[4..]).into();
            } else if line.starts_with(b"MClass:") {
                mclass = String::from_utf8_lossy(&line[7..]).into();
            }
            // TODO load name, it's in euc-kr..
        }

        Self { aid, mclass }
    }
}