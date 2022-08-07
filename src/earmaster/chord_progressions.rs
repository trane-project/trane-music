use std::collections::BTreeMap;

use lazy_static::lazy_static;
use trane::course_builder::CourseBuilder;
use ustr::Ustr;

use super::{EarMasterCourse, EarMasterLesson};

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::music::earmaster::chord_progressions");
}

pub fn course_builder() -> CourseBuilder {
    let earmaster_course = EarMasterCourse {
        id: *COURSE_ID,
        name: "Chord Progressions".to_string(),
        directory_name: "earmaster_chord_progressions".to_string(),
        metadata: Some(BTreeMap::from([])),
        lessons: vec![
            EarMasterLesson::new("1.1", "Identify major and minor tonic in V-1", vec![]),
            EarMasterLesson::new("1.2", "Dominants with and without 7", vec!["1.1"]),
            EarMasterLesson::new("1.3", "VIIdim7 - in major and minor", vec!["1.2"]),
            EarMasterLesson::new("1.4", "Common dominant -tonic combinations", vec!["1.3"]),
            EarMasterLesson::new(
                "1.5",
                "Major/minor subdominant (IV) and supertonic (II)",
                vec!["1.4"],
            ),
            EarMasterLesson::new("1.6", "Various dominant-tonic combinations", vec!["1.5"]),
            EarMasterLesson::new("1.7", "II - V - I combinations", vec!["1.6"]),
            EarMasterLesson::new(
                "1.8",
                "II - V - I  and II - bII - I combinations",
                vec!["1.7"],
            ),
            EarMasterLesson::new("1.9", "IV - I combinations (plagal cadences)", vec!["1.8"]),
            EarMasterLesson::new("1.10", "Two mediant combinations", vec!["1.9"]),
            EarMasterLesson::new("1.11", "Four mediant combinations", vec!["1.10"]),
            EarMasterLesson::new("1.12", "Three part plagal cadences", vec!["1.11"]),
            EarMasterLesson::new("1.13", "Extended tonal cadences", vec!["1.12"]),
            EarMasterLesson::new("1.14", "Fifth sequences", vec!["1.13"]),
            EarMasterLesson::new("1.15", "Folk progessions", vec!["1.14"]),
            EarMasterLesson::new("1.16", "Progressions with bVII7(b5)", vec!["1.15"]),
            EarMasterLesson::new("1.17", "Interrupted cadences", vec!["1.16"]),
            EarMasterLesson::new("1.18", "Modulations #1", vec!["1.17"]),
            EarMasterLesson::new("1.19", "Modulations #2", vec!["1.18"]),
            EarMasterLesson::new("1.20", "Modulations to bIII and bVI", vec!["1.19"]),
            EarMasterLesson::new("1.21", "Last step!", vec!["1.20"]),
        ],
    };
    earmaster_course.course_builder()
}
