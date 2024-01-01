extern crate alloc;

mod audio_sink;
mod database;
mod window;

use std::{
    fs,
    io::stderr,
    time::{SystemTime, UNIX_EPOCH},
};

use clap::Parser;
use winit::keyboard::{KeyCode as WinitKeyCode, PhysicalKey};

use wie_backend::{extract_zip, Archive, Instant, Platform, Screen, System};
use wie_base::{Event, KeyCode};
use wie_ktf::KtfArchive;
use wie_lgt::LgtArchive;
use wie_skt::SktArchive;

use self::{
    audio_sink::AudioSink,
    database::DatabaseRepository,
    window::{WindowCallbackEvent, WindowImpl},
};

struct WieCliPlatform {
    database_repository: DatabaseRepository,
    window: Box<dyn Screen>,
}

impl WieCliPlatform {
    fn new(app_id: &str, window: Box<dyn Screen>) -> Self {
        Self {
            database_repository: DatabaseRepository::new(app_id),
            window,
        }
    }
}

impl Platform for WieCliPlatform {
    fn screen(&mut self) -> &mut dyn Screen {
        self.window.as_mut()
    }

    fn now(&self) -> Instant {
        let now = SystemTime::now();
        let since_the_epoch = now.duration_since(UNIX_EPOCH).unwrap();

        Instant::from_epoch_millis(since_the_epoch.as_millis() as _)
    }

    fn database_repository(&self) -> &dyn wie_backend::DatabaseRepository {
        &self.database_repository
    }

    fn audio_sink(&self) -> Box<dyn wie_backend::AudioSink> {
        Box::new(AudioSink)
    }
}

#[derive(Parser)]
struct Args {
    filename: String,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_writer(stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    start(&Args::parse().filename)
}

pub fn start(filename: &str) -> anyhow::Result<()> {
    let buf = fs::read(filename)?;

    let files = extract_zip(&buf)?;

    let archive: Box<dyn Archive> = if KtfArchive::is_ktf_archive(&files) {
        Box::new(KtfArchive::from_zip(files)?)
    } else if LgtArchive::is_lgt_archive(&files) {
        Box::new(LgtArchive::from_zip(files)?)
    } else if SktArchive::is_skt_archive(&files) {
        Box::new(SktArchive::from_zip(files)?)
    } else {
        anyhow::bail!("Unknown archive format");
    };

    let window = WindowImpl::new(240, 320).unwrap(); // TODO hardcoded size
    let platform = WieCliPlatform::new(&archive.id(), Box::new(window.handle()));

    let mut system = System::new(Box::new(platform));
    let mut system_handle = system.handle();
    let mut app = archive.load_app(&mut system_handle)?;

    app.start()?;

    window.run(move |event| {
        match event {
            WindowCallbackEvent::Update => system.tick().map_err(|x| anyhow::anyhow!("{}\n{}", x, app.crash_dump()))?,
            WindowCallbackEvent::Redraw => system_handle.event_queue().push(Event::Redraw),
            WindowCallbackEvent::Keydown(x) => {
                if let Some(keycode) = convert_key(x) {
                    system_handle.event_queue().push(Event::Keydown(keycode));
                }
            }
            WindowCallbackEvent::Keyup(x) => {
                if let Some(keycode) = convert_key(x) {
                    system_handle.event_queue().push(Event::Keyup(keycode));
                }
            }
        }

        anyhow::Ok(())
    })
}

fn convert_key(key: PhysicalKey) -> Option<KeyCode> {
    match key {
        PhysicalKey::Code(WinitKeyCode::Digit1) => Some(KeyCode::NUM1),
        PhysicalKey::Code(WinitKeyCode::Digit2) => Some(KeyCode::NUM2),
        PhysicalKey::Code(WinitKeyCode::Digit3) => Some(KeyCode::NUM3),
        PhysicalKey::Code(WinitKeyCode::KeyQ) => Some(KeyCode::NUM4),
        PhysicalKey::Code(WinitKeyCode::KeyW) => Some(KeyCode::NUM5),
        PhysicalKey::Code(WinitKeyCode::KeyE) => Some(KeyCode::NUM6),
        PhysicalKey::Code(WinitKeyCode::KeyA) => Some(KeyCode::NUM7),
        PhysicalKey::Code(WinitKeyCode::KeyS) => Some(KeyCode::NUM8),
        PhysicalKey::Code(WinitKeyCode::KeyD) => Some(KeyCode::NUM9),
        PhysicalKey::Code(WinitKeyCode::KeyZ) => Some(KeyCode::STAR),
        PhysicalKey::Code(WinitKeyCode::KeyX) => Some(KeyCode::NUM0),
        PhysicalKey::Code(WinitKeyCode::KeyC) => Some(KeyCode::HASH),
        PhysicalKey::Code(WinitKeyCode::Space) => Some(KeyCode::OK),
        PhysicalKey::Code(WinitKeyCode::ArrowUp) => Some(KeyCode::UP),
        PhysicalKey::Code(WinitKeyCode::ArrowDown) => Some(KeyCode::DOWN),
        PhysicalKey::Code(WinitKeyCode::ArrowLeft) => Some(KeyCode::LEFT),
        PhysicalKey::Code(WinitKeyCode::ArrowRight) => Some(KeyCode::RIGHT),
        _ => None,
    }
}