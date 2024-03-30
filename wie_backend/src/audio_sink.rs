pub trait AudioSink: Sync + Send {
    fn play_wave(&self, channel: u8, sampling_rate: u32, wave_data: &[i16]);
}
