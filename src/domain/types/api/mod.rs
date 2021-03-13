//! Our API root module
pub mod match_info_response;
pub use match_info_response::*;

use serde::{
    Deserialize,
    Serialize,
};

use ron::ser::{
    to_writer_pretty,
    PrettyConfig,
};

use std::{
    fs,
    io::{
        BufReader,
        BufWriter,
    },
    path::PathBuf,
};
/// Datastructure for an incoming `request` on our api
/// on the `matchinfo` endpoint
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MatchInfoRequest {
    /// Requested language (Optional), Standard value is "en"
    pub language: Option<String>,
    /// Requested game (Optional), Standard value is "aoe2de"
    pub game: Option<String>,
    /// Requested type of ID, possible values are ["steam_id", "profile_id"]
    pub id_type: String,
    /// The ID itself as a String
    pub id_number: String,
}

impl MatchInfoRequest {
    /// Create a [`MatchInfoRequest`] from a parsed `RON` file
    ///
    /// # Panics
    /// Panics when the file can not be created or data cannot be written to the
    /// file
    #[must_use]
    pub fn new_from_file(path: PathBuf) -> Self {
        let file = fs::File::open(path).expect("file should open read only");
        let reader = BufReader::new(file);
        ron::de::from_reader::<_, Self>(reader).unwrap()
    }

    /// Create a [`MatchInfoRequest`] from a parsed `RON` file
    ///
    /// # Panics
    /// Panics when the file can not be created or data cannot be written to the
    /// file
    #[must_use]
    pub fn new_from_folder(path: PathBuf) -> Self {
        let mut file_path = path.clone();
        file_path.push("match_info_request.ron");
        ron::de::from_reader::<_, Self>(BufReader::new(
            fs::File::open(file_path).expect("file should open read only"),
        ))
        .unwrap()
    }

    /// Write a RON file of [`MatchInfoRequest`] to
    /// `logs/match_info_request.ron` for debugging purposes
    ///
    /// # Panics
    /// Panics when the file can not be created or data cannot be written to the
    /// file
    pub fn export_data_to_file(
        &self,
        path: PathBuf,
    ) {
        let ron_config = PrettyConfig::new()
            .with_depth_limit(8)
            .with_separate_tuple_members(true)
            .with_enumerate_arrays(true)
            .with_indentor("\t".to_owned());

        let mut assembly_path = PathBuf::new();
        assembly_path.push(path);
        assembly_path.push("match_info_request.ron".to_string());

        // Open the file in writable mode with buffer.
        let file = fs::File::create(assembly_path).unwrap();
        let writer = BufWriter::new(file);

        // Write data to file
        to_writer_pretty(writer, &self, ron_config)
            .expect("Unable to write data");
    }
}
