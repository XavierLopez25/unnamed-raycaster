use lazy_static::lazy_static;
use rodio::Sink;
use rodio::Source;
use std::fs::File;
use std::io::{BufReader, Read};
use std::sync::Mutex;

lazy_static! {
    static ref FOOTSTEP_SINK: Mutex<Option<Sink>> = Mutex::new(None);
}

pub fn play_footstep_sound(
    stream_handle: &rodio::OutputStreamHandle,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("assets\\walking.mp3")?;
    let buf_reader = BufReader::new(file);
    let source = rodio::Decoder::new(buf_reader)?.convert_samples::<f32>();
    let mut sink_guard = FOOTSTEP_SINK.lock().unwrap();
    if let Some(sink) = sink_guard.as_ref() {
        sink.stop();
    }
    let sink = Sink::try_new(stream_handle)?;
    sink.append(source);
    *sink_guard = Some(sink);
    Ok(())
}
