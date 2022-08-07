use std::collections::BTreeMap;

use lazy_static::lazy_static;
use trane::course_builder::CourseBuilder;
use ustr::Ustr;

use super::{EarMasterCourse, EarMasterLesson};

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::music::earmaster::scale_identification");
}

pub fn course_builder() -> CourseBuilder {
    let earmaster_course = EarMasterCourse {
        id: *COURSE_ID,
        name: "Scale Identification".to_string(),
        directory_name: "earmaster_scale_identification".to_string(),
        metadata: Some(BTreeMap::from([])),
        lessons: vec![
            EarMasterLesson::new("1.1", "Major, Lydian and Mixolydian", vec![]),
            EarMasterLesson::new("1.2", "Scales with minor sound", vec!["1.1"]),
            EarMasterLesson::new("1.3", "Phrygian and Locrian", vec!["1.2"]),
            EarMasterLesson::new(
                "1.4",
                "All modes and harmonic and melodic minor",
                vec!["1.3"],
            ),
            EarMasterLesson::new(
                "1.5",
                "Whole tone and variants of Lydian and Mixolydian",
                vec!["1.4"],
            ),
            EarMasterLesson::new("1.6", "Melodic minor, Locrian (#2) and Dim", vec!["1.5"]),
            EarMasterLesson::new("1.7", "Phrygian (#6), Altered and Dim", vec!["1.6"]),
            EarMasterLesson::new(
                "1.8",
                "Derivatives of melodic minor, Whole tone and Dim",
                vec!["1.7"],
            ),
            EarMasterLesson::new("1.9", "All scales and modes from lessons 1-8", vec!["1.8"]),
            EarMasterLesson::new("1.10", "Derivatives of harmonic minor", vec!["1.9"]),
            EarMasterLesson::new("1.11", "The two pentatonic scales", vec!["1.10"]),
            EarMasterLesson::new("1.12", "All modes - descending", vec!["1.11"]),
            EarMasterLesson::new(
                "1.13",
                "Derivatives of harmonic minor - descending",
                vec!["1.12"],
            ),
        ],
    };
    earmaster_course.course_builder()
}
