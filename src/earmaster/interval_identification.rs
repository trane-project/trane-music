use std::collections::BTreeMap;

use lazy_static::lazy_static;
use trane::course_builder::CourseBuilder;
use ustr::Ustr;

use super::{EarMasterCourse, EarMasterLesson};

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::music::earmaster::interval_identification");
}

pub fn course_builder() -> CourseBuilder {
    let earmaster_course = EarMasterCourse {
        id: *COURSE_ID,
        name: "Interval Identification".to_string(),
        directory_name: "earmaster_interval_identification".to_string(),
        metadata: Some(BTreeMap::from([])),
        lessons: vec![
            // Unit 1.
            EarMasterLesson::new("1.1", "Minor 2nd & Major 2nd - Ascending", vec![]),
            EarMasterLesson::new("1.2", "Minor 2nd & Major 2nd - Descending", vec![]),
            EarMasterLesson::new("1.3", "Minor 2nd & Major 2nd - Harmonic", vec![]),
            EarMasterLesson::new(
                "1.4",
                "Minor 2nd & Major 2nd - Ascending, Descending & Harmonic",
                vec!["1.1", "1.2", "1.3"],
            ),
            // Unit 2.
            EarMasterLesson::new("2.1", "Minor 3rd & Major 3rd - Ascending", vec![]),
            EarMasterLesson::new("2.2", "Minor 3rd & Major 3rd - Descending", vec![]),
            EarMasterLesson::new("2.3", "Minor 3rd & Major 3rd - Harmonic", vec![]),
            EarMasterLesson::new(
                "2.4",
                "Minor 3rd & Major 3rd - Ascending, Descending & Harmonic",
                vec!["2.1", "2.2", "2.3"],
            ),
            // Unit 3.
            EarMasterLesson::new(
                "3.1",
                "All intervals from Minor 2nd to Major 3rd - Ascending",
                vec!["1.4", "2.4"],
            ),
            EarMasterLesson::new(
                "3.2",
                "All intervals from Minor 2nd to Major 3rd - Descending",
                vec!["1.4", "2.4"],
            ),
            EarMasterLesson::new(
                "3.3",
                "All intervals from Minor 2nd to Major 3rd - Harmonic",
                vec!["1.4", "2.4"],
            ),
            EarMasterLesson::new(
                "3.4",
                "All Intervals from Minor 2nd to Major 3rd - Ascending, Descending & Harmonic",
                vec!["3.1", "3.2", "3.3"],
            ),
            // Unit 4.
            EarMasterLesson::new("4.1", "Perfect 4th & Dim 5th - Ascending", vec!["3.4"]),
            EarMasterLesson::new("4.2", "Perfect 4th & Dim 5th - Descending", vec!["3.4"]),
            EarMasterLesson::new("4.3", "Perfect 4th & Dim 5th - Harmonic", vec!["3.4"]),
            EarMasterLesson::new(
                "4.4",
                "Perfect 4th & Dim 5th - Ascending, Descending & Harmonic",
                vec!["4.1", "4.2", "4.3"],
            ),
            // Unit 5.
            EarMasterLesson::new("5.1", "Dim 5th & Perfect 5th - Ascending", vec!["3.4"]),
            EarMasterLesson::new("5.2", "Dim 5th & Perfect 5th - Descending", vec!["3.4"]),
            EarMasterLesson::new("5.3", "Dim 5th & Perfect 5th - Harmonic", vec!["3.4"]),
            EarMasterLesson::new(
                "5.4",
                "Dim 5th & Perfect 5th - Ascending, Descending & Harmonic",
                vec!["5.1", "5.2", "5.3"],
            ),
            // Unit 6.
            EarMasterLesson::new(
                "6.1",
                "Perfect 4th, Dim 5th & Perfect 5th - Ascending",
                vec!["4.4", "5.4"],
            ),
            EarMasterLesson::new(
                "6.2",
                "Perfect 4th, Dim 5th & Perfect 5th - Descending",
                vec!["4.4", "5.4"],
            ),
            EarMasterLesson::new(
                "6.3",
                "Perfect 4th, Dim 5th & Perfect 5th - Harmonic",
                vec!["4.4", "5.4"],
            ),
            EarMasterLesson::new(
                "6.4",
                "Perfect 4th, Dim 5th & Perfect 5th - Ascending, Descending & Harmonic",
                vec!["6.1", "6.2", "6.3"],
            ),
            // Unit 7.
            EarMasterLesson::new(
                "7.1",
                "All intervals from Minor 2nd to Perfect 5th - Ascending",
                vec!["6.4"],
            ),
            EarMasterLesson::new(
                "7.2",
                "All intervals from Minor 2nd to Perfect 5th - Descending",
                vec!["6.4"],
            ),
            EarMasterLesson::new(
                "7.3",
                "All intervals from Minor 2nd to Perfect 5th - Harmonic",
                vec!["6.4"],
            ),
            EarMasterLesson::new(
                "7.4",
                "All intervals from Minor 2nd to Perfect 5th - Ascending, Descending & Harmonic",
                vec!["7.1", "7.2", "7.3"],
            ),
            // Unit 8.
            EarMasterLesson::new("8.1", "Major 6th & Minor 6th - Ascending", vec!["7.4"]),
            EarMasterLesson::new("8.2", "Major 6th & Minor 6th - Descending", vec!["7.4"]),
            EarMasterLesson::new("8.3", "Major 6th & Minor 6th - Harmonic", vec!["7.4"]),
            EarMasterLesson::new(
                "8.4",
                "Major 6th & Minor 6th - Ascending, Descending & Harmonic",
                vec!["8.1", "8.2", "8.3"],
            ),
            // Unit 9.
            EarMasterLesson::new("9.1", "Major 7th & Minor 7th - Ascending", vec!["7.4"]),
            EarMasterLesson::new("9.2", "Major 7th & Minor 7th - Descending", vec!["7.4"]),
            EarMasterLesson::new("9.3", "Major 7th & Minor 7th - Harmonic", vec!["7.4"]),
            EarMasterLesson::new(
                "9.4",
                "Major 7th & Minor 7th - Ascending, Descending & Harmonic",
                vec!["9.1", "9.2", "9.3"],
            ),
            // Unit 10.
            EarMasterLesson::new(
                "10.1",
                "All intervals from Minor 6th to Major 7th - Ascending",
                vec!["8.4", "9.4"],
            ),
            EarMasterLesson::new(
                "10.2",
                "All intervals from Minor 6th to Major 7th - Descending",
                vec!["8.4", "9.4"],
            ),
            EarMasterLesson::new(
                "10.3",
                "All intervals from Minor 6th to Major 7th - Harmonic",
                vec!["8.4", "9.4"],
            ),
            EarMasterLesson::new(
                "10.4",
                "All intervals from Minor 6th to Major 7th - Ascending, Descending & Harmonic",
                vec!["10.1", "10.2", "10.3"],
            ),
            // Unit 11.
            EarMasterLesson::new(
                "11.1",
                "All intervals from Perfect 4th to Major 7th - Ascending",
                vec!["10.4"],
            ),
            EarMasterLesson::new(
                "11.2",
                "All intervals from Perfect 4th to Major 7th - Descending",
                vec!["10.4"],
            ),
            EarMasterLesson::new(
                "11.3",
                "All intervals from Perfect 4th to Major 7th - Harmonic",
                vec!["10.4"],
            ),
            EarMasterLesson::new(
                "11.4",
                "All intervals from Perfect 4th to Major 7th - Ascending, Descending & Harmonic",
                vec!["11.1", "11.2", "11.3"],
            ),
            // Unit 12.
            EarMasterLesson::new(
                "12.1",
                "All intervals from Minor 2nd to Major 7th - Ascending",
                vec!["11.4"],
            ),
            EarMasterLesson::new(
                "12.2",
                "All intervals from Minor 2nd to Major 7th - Descending",
                vec!["11.4"],
            ),
            EarMasterLesson::new(
                "12.3",
                "All intervals from Minor 2nd to Major 7th - Harmonic",
                vec!["11.4"],
            ),
            EarMasterLesson::new(
                "12.4",
                "All intervals from Minor 2nd to Major 7th - Ascending, Descending & Harmonic",
                vec!["12.1", "12.2", "12.3"],
            ),
            // Unit 13.
            EarMasterLesson::new("13.1", "Unison & Octave - Harmonic", vec!["12.4"]),
            EarMasterLesson::new(
                "13.2",
                "Unison & Octave - Ascending, Descending & Harmonic",
                vec!["13.1"],
            ),
            EarMasterLesson::new("13.3", "Perfect 4th & Perfect 5th - Harmonic", vec!["12.4"]),
            EarMasterLesson::new(
                "13.4",
                "Perfect 4th & Perfect 5th - Ascending, Descending & Harmonic",
                vec!["13.3"],
            ),
            EarMasterLesson::new(
                "13.5",
                "Unison, Perfect 4th, Perfect 5th & Octave - Ascending",
                vec!["13.2", "13.4"],
            ),
            EarMasterLesson::new(
                "13.6",
                "Unison, Perfect 4th, Perfect 5th & Octave - Descending",
                vec!["13.2", "13.4"],
            ),
            EarMasterLesson::new(
                "13.7",
                "Unison, Perfect 4th, Perfect 5th & Octave - Harmonic",
                vec!["13.2", "13.4"],
            ),
            EarMasterLesson::new(
                "13.8",
                "Unison, Perfect 4th, Perfect 5th & Octave - Ascending, Descending & Harmonic",
                vec!["13.5", "13.6", "13.7"],
            ),
            // Unit 14.
            EarMasterLesson::new("14.1", "Minor 3rd & Major 6th - Harmonic", vec!["12.4"]),
            EarMasterLesson::new(
                "14.2",
                "Minor 3rd & Major 6th - Ascending, Descending & Harmonic",
                vec!["14.1"],
            ),
            EarMasterLesson::new("14.3", "Major 3rd & Minor 6th - Harmonic", vec!["12.4"]),
            EarMasterLesson::new(
                "14.4",
                "Major 3rd & Minor 6th - Ascending, Descending & Harmonic",
                vec!["14.3"],
            ),
            EarMasterLesson::new(
                "14.5",
                "Minor 3rd, Major 3rd, Minor 6th & Major 6th - Ascending",
                vec!["14.2", "14.4"],
            ),
            EarMasterLesson::new(
                "14.6",
                "Minor 3rd, Major 3rd, Minor 6th & Major 6th - Descending",
                vec!["14.2", "14.4"],
            ),
            EarMasterLesson::new(
                "14.7",
                "Minor 3rd, Major 3rd, Minor 6th & Major 6th - Harmonic",
                vec!["14.2", "14.4"],
            ),
            EarMasterLesson::new(
                "14.8",
                "Minor 3rd, Major 3rd, Minor 6th & Major 6th - Ascending, Descending & Harmonic",
                vec!["14.5", "14.6", "14.7"],
            ),
            // Unit 15.
            EarMasterLesson::new("15.1", "Minor 2nd & Major 7th - Harmonic", vec!["12.4"]),
            EarMasterLesson::new(
                "15.2",
                "Minor 2nd & Major 7th - Ascending, Descending & Harmonic",
                vec!["15.1"],
            ),
            EarMasterLesson::new("15.3", "Major 2nd & Minor 7th - Harmonic", vec!["12.4"]),
            EarMasterLesson::new(
                "15.4",
                "Major 2nd & Minor 7th - Ascending, Descending & Harmonic",
                vec!["15.3"],
            ),
            EarMasterLesson::new(
                "15.5",
                "Minor 2nd, Major 2nd, Minor 7th & Major 7th - Ascending",
                vec!["15.2", "15.4"],
            ),
            EarMasterLesson::new(
                "15.6",
                "Minor 2nd, Major 2nd, Minor 7th & Major 7th - Descending",
                vec!["15.2", "15.4"],
            ),
            EarMasterLesson::new(
                "15.7",
                "Minor 2nd, Major 2nd, Minor 7th & Major 7th - Harmonic",
                vec!["15.2", "15.4"],
            ),
            EarMasterLesson::new(
                "15.8",
                "Minor 2nd, Major 2nd, Minor 7th & Major 7th - Ascending, Descending & Harmonic",
                vec!["15.5", "15.6", "15.7"],
            ),
            EarMasterLesson::new(
                "15.9",
                "Minor 2nd, Dim 5th & Major 7th - Ascending",
                vec!["15.8"],
            ),
            EarMasterLesson::new(
                "15.10",
                "Minor 2nd, Dim 5th & Major 7th - Descending",
                vec!["15.8"],
            ),
            EarMasterLesson::new(
                "15.11",
                "Minor 2nd, Dim 5th & Major 7th - Harmonic",
                vec!["15.8"],
            ),
            EarMasterLesson::new(
                "15.12",
                "Major 2nd, Dim 5th & Minor 7th - Ascending",
                vec!["15.8"],
            ),
            EarMasterLesson::new(
                "15.13",
                "Major 2nd, Dim 5th & Minor 7th - Descending",
                vec!["15.8"],
            ),
            EarMasterLesson::new(
                "15.14",
                "Major 2nd, Dim 5th & Minor 7th - Harmonic",
                vec!["15.8"],
            ),
            EarMasterLesson::new(
                "15.15",
                "Minor 2nd, Major 2nd, Dim 5th, Minor 7th & Major 7th - Ascending",
                vec!["15.9", "15.10", "15.11", "15.12", "15.13", "15.14"],
            ),
            EarMasterLesson::new(
                "15.16",
                "Minor 2nd, Major 2nd, Dim 5th, Minor 7th & Major 7th - Descending",
                vec!["15.9", "15.10", "15.11", "15.12", "15.13", "15.14"],
            ),
            EarMasterLesson::new(
                "15.17",
                "Minor 2nd, Major 2nd, Dim 5th, Minor 7th & Major 7th - Harmonic",
                vec!["15.9", "15.10", "15.11", "15.12", "15.13", "15.14"],
            ),
            EarMasterLesson::new(
                "15.18",
                "Minor 2nd, Major 2nd, Dim 5th, Minor 7th & Major 7th - Ascending, Descending & \
                 Harmonic",
                vec!["15.15", "15.16", "15.17"],
            ),
            // Unit 16.
            EarMasterLesson::new(
                "16.1",
                "All intervals from Unison to Octave - Ascending",
                vec!["13.8", "14.8", "15.18"],
            ),
            EarMasterLesson::new(
                "16.2",
                "All intervals from Unison to Octave - Descending",
                vec!["16.1"],
            ),
            EarMasterLesson::new(
                "16.3",
                "All intervals from Unison to Octave - Harmonic",
                vec!["16.1"],
            ),
            EarMasterLesson::new(
                "16.4",
                "All intervals from Unison to Octave - Ascending, Descending & Harmonic",
                vec!["16.1", "16.2", "16.3"],
            ),
        ],
    };
    earmaster_course.course_builder()
}
