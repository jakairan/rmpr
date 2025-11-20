use rodio::{Decoder, OutputStreamHandle, PlayError, Sink, decoder::DecoderError};
use std::{
    fmt,
    fs::File,
    io::{self, BufReader},
    path::PathBuf,
    sync::Mutex,
    time::Duration,
};

#[derive(Debug)]
pub enum AudioError {
    Io(io::Error),
    Decoder(DecoderError),
    Play(PlayError),
}

impl fmt::Display for AudioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AudioError::Io(e) => write!(f, "IO error: {}", e),
            AudioError::Decoder(e) => write!(f, "Decoder error: {}", e),
            AudioError::Play(e) => write!(f, "Stream error: {}", e),
        }
    }
}

/// Encapsulates an audio sink and an output stream handle.
pub struct SinkHandler {
    sink: Mutex<Option<Sink>>,
    stream_handle: OutputStreamHandle,
}

impl SinkHandler {
    pub fn new(stream_handle: OutputStreamHandle) -> Self {
        Self {
            stream_handle,
            sink: Mutex::new(None),
        }
    }

    /// Plays the given file and sets its volume.
    pub fn play_file(&self, path: PathBuf, vol: i16) -> Result<(), AudioError> {
        let file = File::open(path).map_err(AudioError::Io)?;
        let reader = BufReader::new(file);
        let source = Decoder::new(reader).map_err(AudioError::Decoder)?;
        let sink = Sink::try_new(&self.stream_handle).map_err(AudioError::Play)?;

        sink.append(source);
        *self.sink.lock().expect("Mutex poisoned") = Some(sink);
        self.set_volume(vol);
        Ok(())
    }

    /// Toggles play and pause.
    pub fn toggle_play_pause(&self) {
        let sink_guard = self.sink.lock().expect("Mutex poisoned");
        if let Some(ref sink) = *sink_guard {
            if sink.is_paused() {
                sink.play()
            } else {
                sink.pause()
            }
        }
    }

    /// Sets the playback volume.
    pub fn set_volume(&self, mag: i16) {
        let sink_guard = self.sink.lock().expect("Mutex poisoned");
        if let Some(ref sink) = *sink_guard {
            sink.set_volume((mag as f32) / 100.0);
        }
    }

    /// Gets the sink's position in seconds.
    pub fn sink_pos(&self) -> u64 {
        let sink_guard = self.sink.lock().expect("Mutex poisoned");
        match &*sink_guard {
            Some(sink) => sink.get_pos().as_secs(),
            None => Duration::new(0, 0).as_secs(),
        }
    }

    /// Gets the sink's position in milliseconds.
    pub fn sink_pos_millis(&self) -> u128 {
        let sink_guard = self.sink.lock().expect("Mutex poisoned");
        match &*sink_guard {
            Some(sink) => sink.get_pos().as_millis(),
            None => Duration::new(0, 0).as_millis(),
        }
    }

    /// Appends source to sink.
    pub fn append_to_sink(&self, path: PathBuf, vol: i16) -> Result<(), AudioError> {
        let file = File::open(path).map_err(AudioError::Io)?;
        let reader = BufReader::new(file);
        let source = Decoder::new(reader).map_err(AudioError::Decoder)?;

        {
            let sink_guard = self.sink.lock().expect("Mutex poisoned");
            if let Some(ref sink) = *sink_guard {
                if 1 <= sink.len() {
                    sink.append(source);
                }
            }
        }

        self.set_volume(vol);
        Ok(())
    }

    /// Removes all currently loaded Sources from the Sink, and pauses it.
    pub fn clear(&self) {
        let sink_guard = self.sink.lock().expect("Mutex poisoned");
        if let Some(ref sink) = *sink_guard {
            sink.clear();
        }
    }

    /// Returns how many elements are in the sink.
    pub fn get_len(&self) -> usize {
        let sink_guard = self.sink.lock().expect("Mutex poisoned");
        match &*sink_guard {
            Some(sink) => sink.len(),
            None => 0,
        }
    }

    /// Returns true if the sink is empty, otherwise false.
    pub fn is_empty(&self) -> bool {
        let sink_guard = self.sink.lock().expect("Mutex poisoned");
        match &*sink_guard {
            Some(sink) => sink.empty(),
            None => true,
        }
    }
}
