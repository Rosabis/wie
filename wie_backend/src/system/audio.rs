use alloc::{boxed::Box, collections::BTreeMap, sync::Arc, vec::Vec};

use smaf_player::{SmafEvent, parse_smaf};

use crate::{System, audio_sink::AudioSink};

pub type AudioHandle = u32;
#[derive(Debug)]
pub enum AudioError {
    InvalidHandle,
    InvalidAudio,
}

enum AudioFile {
    Smaf(Vec<u8>),
}

pub struct Audio {
    sink: Arc<Box<dyn AudioSink>>,
    files: BTreeMap<AudioHandle, AudioFile>,
    last_audio_handle: AudioHandle,
}

impl Audio {
    pub fn new(sink: Box<dyn AudioSink>) -> Self {
        Self {
            sink: Arc::new(sink),
            files: BTreeMap::new(),
            last_audio_handle: 0,
        }
    }

    pub fn load_smaf(&mut self, data: &[u8]) -> Result<AudioHandle, AudioError> {
        let audio_handle = self.last_audio_handle;

        self.last_audio_handle += 1;
        self.files.insert(audio_handle, AudioFile::Smaf(data.to_vec()));

        Ok(audio_handle)
    }

    pub fn play(&self, system: &System, audio_handle: AudioHandle) -> Result<(), AudioError> {
        match self.files.get(&audio_handle) {
            Some(AudioFile::Smaf(data)) => {
                let player = SmafPlayer::new(data);
                let mut system_clone = system.clone();
                let sink_clone = self.sink.clone();

                system.spawn(async move || {
                    player.play(&mut system_clone, &**sink_clone).await;

                    Ok(())
                });
            }
            None => return Err(AudioError::InvalidHandle),
        }

        Ok(())
    }
}

pub struct SmafPlayer {
    events: Vec<(usize, SmafEvent)>,
}

impl SmafPlayer {
    pub fn new(data: &[u8]) -> Self {
        Self { events: parse_smaf(data) }
    }

    pub async fn play(&self, system: &mut System, sink: &dyn AudioSink) {
        let mut play_time = 0;
        for (time, event) in &self.events {
            let now = system.platform().now();
            system.sleep(now + ((time - play_time) as u64)).await;

            match event {
                SmafEvent::Wave {
                    channel,
                    sampling_rate,
                    data,
                } => {
                    sink.play_wave(*channel, *sampling_rate, data);
                }
                SmafEvent::MidiNoteOn { channel, note, velocity } => {
                    sink.midi_note_on(*channel, *note, *velocity);
                }
                SmafEvent::MidiNoteOff { channel, note, velocity } => {
                    sink.midi_note_off(*channel, *note, *velocity);
                }
                SmafEvent::MidiProgramChange { channel, program } => {
                    sink.midi_program_change(*channel, *program);
                }
                SmafEvent::MidiControlChange { channel, control, value } => {
                    sink.midi_control_change(*channel, *control, *value);
                }
                SmafEvent::End => {}
            }

            play_time = *time;
        }
    }
}
