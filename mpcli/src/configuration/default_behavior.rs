use std::io;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DefaultBehavior {
    pub quiet: bool,
    pub extract_title: bool,
}

impl DefaultBehavior {
    pub fn new(quiet: bool, extract_title: bool) -> Self {
        DefaultBehavior { quiet, extract_title }
    }

    pub fn from_args(quiet: String, extract_title: String) -> io::Result<Self> {
        let is_quiet = quiet.trim().eq_ignore_ascii_case("y");
        let should_extract_title = extract_title.trim().eq_ignore_ascii_case("y");

        Ok(DefaultBehavior::new(is_quiet, should_extract_title))
    }
}