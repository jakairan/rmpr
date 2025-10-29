use audiotags::Tag;
use std::path::PathBuf;

/// Encapsulates file data information.
#[derive(Clone)]
pub struct FileMetadata {
    pub album: Option<String>,
    pub artist: Option<String>,
    pub duration_as_secs: Option<f64>,
    pub duration_display: Option<(f64, f64)>,
    pub raw_file: String,
    pub title: Option<String>,
    pub track_number: Option<u16>,
    pub year: Option<i32>,
}

impl FileMetadata {
    pub fn new() -> Self {
        Self {
            raw_file: String::new(),
            album: None,
            artist: None,
            title: None,
            year: None,
            duration_display: None,
            duration_as_secs: None,
            track_number: None,
        }
    }

    fn with_file_only(raw_file: String) -> Self {
        Self {
            raw_file,
            album: None,
            artist: None,
            title: None,
            year: None,
            duration_display: None,
            duration_as_secs: None,
            track_number: None,
        }
    }

    /// Sets FileMetadata with the respective values from the file.
    pub fn get_file_data(path: &PathBuf) -> FileMetadata {
        let valid_exts = ["flac", "mp3", "m4a", "mp4"];
        let raw_file = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or("Unknown".to_string());

        let Some(ext) = path.extension() else {
            return Self::with_file_only(raw_file);
        };

        let file_ext = ext.to_string_lossy().to_ascii_lowercase();
        if !valid_exts.contains(&file_ext.as_str()) {
            return Self::with_file_only(raw_file);
        }

        let Ok(tags) = Tag::default().read_from_path(path) else {
            return Self::with_file_only(raw_file);
        };

        Self {
            raw_file,
            album: tags.album_title().map(|n| n.to_string()),
            artist: tags.artist().map(|n| n.to_string()),
            title: tags.title().map(|n| n.to_string()),
            year: tags.year(),
            duration_display: tags.duration().map(FileMetadata::sec_to_min_sec),
            duration_as_secs: tags.duration(),
            track_number: tags.track_number(),
        }
    }

    /// Display album or nothing.
    pub fn display_album(&self) -> String {
        match &self.album {
            Some(display) => format!("{}", display),
            None => String::new(),
        }
    }

    /// Display artists or nothing.
    pub fn display_artist(&self) -> String {
        match &self.artist {
            Some(artist) => format!("{}", artist),
            None => String::new(),
        }
    }

    /// Display title, or raw file, or nothing if neither is found.
    pub fn display_title(&self) -> String {
        match &self.title {
            Some(title) => format!("{}", title),
            None => format!("{}", self.raw_file),
        }
    }

    /// Display year or nothing.
    pub fn display_year(&self) -> String {
        match self.year {
            Some(year) => format!("{}", year),
            None => String::new(),
        }
    }

    /// Display track_number or nothing.
    pub fn display_track_number(&self) -> String {
        match self.track_number {
            Some(track_number) => format!("{}", track_number),
            None => String::new(),
        }
    }

    /// Converts seconds to seconds and minutes.
    fn sec_to_min_sec(duration: f64) -> (f64, f64) {
        let min = (duration / 60.0).floor();
        let sec = (duration % 60.0).floor();
        (min, sec)
    }

    /// Display duration_display or nothing.
    pub fn display_duration_display(&self) -> String {
        match self.duration_display {
            Some((min, sec)) => format!("{:.0}:{:02.0}", min, sec),
            None => String::new(),
        }
    }
}
