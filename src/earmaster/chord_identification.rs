use std::collections::BTreeMap;

use lazy_static::lazy_static;
use trane::course_builder::CourseBuilder;
use ustr::Ustr;

use super::{EarMasterCourse, EarMasterLesson};

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::music::earmaster::chord_identification");
}

pub fn course_builder() -> CourseBuilder {
    let earmaster_course = EarMasterCourse {
        id: *COURSE_ID,
        name: "Chord Identification".to_string(),
        directory_name: "earmaster_chord_identification".to_string(),
        metadata: Some(BTreeMap::from([])),
        lessons: vec![
            // Unit 1.
            EarMasterLesson::new("1.1", "Major & Minor - Ascending", vec![]),
            EarMasterLesson::new("1.2", "Major & Minor - Descending", vec![]),
            EarMasterLesson::new("1.3", "Major & Minor - Harmonic", vec![]),
            EarMasterLesson::new(
                "1.4",
                "Major & Minor - Harmonic, Ascending & Descending",
                vec!["1.1", "1.2", "1.3"],
            ),
            // Unit 2.
            EarMasterLesson::new("2.1", "Major, Minor & Sus4 - Ascending", vec!["1.4"]),
            EarMasterLesson::new("2.2", "Major, Minor & Sus4 - Descending", vec!["1.4"]),
            EarMasterLesson::new("2.3", "Major, Minor & Sus4 - Harmonic", vec!["1.4"]),
            EarMasterLesson::new(
                "2.4",
                "Major, Minor & Sus4 - Harmonic, Ascending & Descending",
                vec!["2.1", "2.2", "2.3"],
            ),
            // Unit 3.
            EarMasterLesson::new("3.1", "Major, Major(b5) & Aug - Ascending", vec!["2.4"]),
            EarMasterLesson::new("3.2", "Major, Major(b5) & Aug - Descending", vec!["2.4"]),
            EarMasterLesson::new("3.3", "Major, Major(b5) & Aug - Harmonic", vec!["2.4"]),
            EarMasterLesson::new(
                "3.4",
                "Major, Major(b5) & Aug - Harmonic, Ascending & Descending",
                vec!["3.1", "3.2", "3.3"],
            ),
            // Unit 4.
            EarMasterLesson::new("4.1", "Minor & Dim - Ascending", vec!["3.4"]),
            EarMasterLesson::new("4.2", "Minor & Dim - Descending", vec!["3.4"]),
            EarMasterLesson::new("4.3", "Minor & Dim - Harmonic", vec!["3.4"]),
            EarMasterLesson::new(
                "4.4",
                "Minor & Dim - Harmonic, Ascending & Descending",
                vec!["4.1", "4.2", "4.3"],
            ),
            // Unit 5.
            EarMasterLesson::new("5.1", "Sus2 & Sus4 - Ascending", vec!["4.4"]),
            EarMasterLesson::new("5.2", "Sus2 & Sus4 - Descending", vec!["4.4"]),
            EarMasterLesson::new("5.3", "Sus2 & Sus4 - Harmonic", vec!["4.4"]),
            EarMasterLesson::new(
                "5.4",
                "Sus2 & Sus4 - Harmonic, Ascending & Descending",
                vec!["5.1", "5.2", "5.3"],
            ),
            // Unit 6.
            EarMasterLesson::new("6.1", "Dim, Major(b5) & Aug - Ascending", vec!["5.4"]),
            EarMasterLesson::new("6.2", "Dim, Major(b5) & Aug - Descending", vec!["5.4"]),
            EarMasterLesson::new("6.3", "Dim, Major(b5) & Aug - Harmonic", vec!["5.4"]),
            EarMasterLesson::new(
                "6.4",
                "Dim, Major(b5) & Aug - Harmonic, Ascending & Descending",
                vec!["6.1", "6.2", "6.3"],
            ),
            // Unit 7.
            EarMasterLesson::new(
                "7.1",
                "Major, Minor, Dim, Major(b5), Aug, Sus2 & Sus4 - Ascending",
                vec!["6.4"],
            ),
            EarMasterLesson::new(
                "7.2",
                "Major, Minor, Dim, Major(b5), Aug, Sus2 & Sus4 - Descending",
                vec!["6.4"],
            ),
            EarMasterLesson::new(
                "7.3",
                "Major, Minor, Dim, Major(b5), Aug, Sus2 & Sus4 - Harmonic",
                vec!["6.4"],
            ),
            EarMasterLesson::new(
                "7.4",
                "Major, Minor, Dim, Major(b5), Aug, Sus2 & Sus4 - Harmonic, Ascending & Descending",
                vec!["7.1", "7.2", "7.3"],
            ),
            // Unit 8.
            EarMasterLesson::new("8.1", "7, Maj7 & Add6 - Ascending", vec!["7.4"]),
            EarMasterLesson::new("8.2", "7, Maj7 & Add6 - Descending", vec!["7.4"]),
            EarMasterLesson::new("8.3", "7, Maj7 & Add6 - Harmonic", vec!["7.4"]),
            EarMasterLesson::new(
                "8.4",
                "7, Maj7 & Add6 - Harmonic, Ascending & Descending",
                vec!["8.1", "8.2", "8.3"],
            ),
            // Unit 9.
            EarMasterLesson::new("9.1", "Mi7, Mi,maj7 & Mi,add6 - Ascending", vec!["7.4"]),
            EarMasterLesson::new("9.2", "Mi7, Mi,maj7 & Mi,add6 - Descending", vec!["7.4"]),
            EarMasterLesson::new("9.3", "Mi7, Mi,maj7 & Mi,add6 - Harmonic", vec!["7.4"]),
            EarMasterLesson::new(
                "9.4",
                "Mi7, Mi,maj7 & Mi,add6 - Harmonic, Ascending & Descending",
                vec!["9.1", "9.2", "9.3"],
            ),
            // Unit 10.
            EarMasterLesson::new("10.1", "Dim7, Mi7(b5) & dim,maj7 - Ascending", vec!["7.4"]),
            EarMasterLesson::new("10.2", "Dim7, Mi7(b5) & dim,maj7 - Descending", vec!["7.4"]),
            EarMasterLesson::new("10.3", "Dim7, Mi7(b5) & dim,maj7 - Harmonic", vec!["7.4"]),
            EarMasterLesson::new(
                "10.4",
                "Dim7, Mi7(b5) & dim,maj7 - Harmonic, Ascending & Descending",
                vec!["10.1", "10.2", "10.3"],
            ),
            // Unit 11.
            EarMasterLesson::new("11.1", "7(b5) & maj7(b5) - Ascending", vec!["7.4"]),
            EarMasterLesson::new("11.2", "7(b5) & maj7(b5) - Descending", vec!["7.4"]),
            EarMasterLesson::new("11.3", "7(b5) & maj7(b5) - Harmonic", vec!["7.4"]),
            EarMasterLesson::new(
                "11.4",
                "7(b5) & maj7(b5) - Harmonic, Ascending & Descending",
                vec!["11.1", "11.2", "11.3"],
            ),
            // Unit 12.
            EarMasterLesson::new("12.1", "7(#5) & maj7(#5) - Ascending", vec!["7.4"]),
            EarMasterLesson::new("12.2", "7(#5) & maj7(#5) - Descending", vec!["7.4"]),
            EarMasterLesson::new("12.3", "7(#5) & maj7(#5) - Harmonic", vec!["7.4"]),
            EarMasterLesson::new(
                "12.4",
                "7(#5) & maj7(#5) - Harmonic, Ascending & Descending",
                vec!["11.1", "12.2", "12.3"],
            ),
            // Unit 13.
            EarMasterLesson::new("13.1", "7(sus4) & 7(sus2) - Ascending", vec!["7.4"]),
            EarMasterLesson::new("13.2", "7(sus4) & 7(sus2) - Descending", vec!["7.4"]),
            EarMasterLesson::new("13.3", "7(sus4) & 7(sus2) - Harmonic", vec!["7.4"]),
            EarMasterLesson::new(
                "13.4",
                "7(sus4) & 7(sus2) - Harmonic, Ascending & Descending",
                vec!["13.1", "13.2", "13.3"],
            ),
            // Unit 14.
            EarMasterLesson::new("14.1", "maj7(sus2) & maj7(sus4) - Ascending", vec!["7.4"]),
            EarMasterLesson::new("14.2", "maj7(sus2) & maj7(sus4) - Descending", vec!["7.4"]),
            EarMasterLesson::new("14.3", "maj7(sus2) & maj7(sus4) - Harmonic", vec!["7.4"]),
            EarMasterLesson::new(
                "14.4",
                "maj7(sus2) & maj7(sus4) - Harmonic, Ascending & Descending",
                vec!["14.1", "14.2", "14.3"],
            ),
            // Unit 15.
            EarMasterLesson::new(
                "15.1",
                "7(sus4), 7(sus2), maj7(sus2) & maj7(sus4) - Ascending",
                vec!["13.4", "14.4"],
            ),
            EarMasterLesson::new(
                "15.2",
                "7(sus4), 7(sus2), maj7(sus2) & maj7(sus4) - Descending",
                vec!["13.4", "14.4"],
            ),
            EarMasterLesson::new(
                "15.3",
                "7(sus4), 7(sus2), maj7(sus2) & maj7(sus4) - Harmonic",
                vec!["13.4", "14.4"],
            ),
            EarMasterLesson::new(
                "15.4",
                "7(sus4), 7(sus2), maj7(sus2) & maj7(sus4) - Harmonic, Ascending & Descending",
                vec!["15.1", "15.2", "15.3"],
            ),
            // Unit 16.
            EarMasterLesson::new(
                "16.1",
                "Maj7, maj7(b5) & maj7(#5) - Ascending",
                vec!["8.4", "11.4", "12.4"],
            ),
            EarMasterLesson::new(
                "16.2",
                "Maj7, maj7(b5) & maj7(#5) - Descending",
                vec!["8.4", "11.4", "12.4"],
            ),
            EarMasterLesson::new(
                "16.3",
                "Maj7, maj7(b5) & maj7(#5) - Harmonic",
                vec!["8.4", "11.4", "12.4"],
            ),
            EarMasterLesson::new(
                "16.4",
                "Maj7, maj7(b5) & maj7(#5) - Harmonic, Ascending & Descending",
                vec!["16.1", "16.2", "16.3"],
            ),
            // Unit 17.
            EarMasterLesson::new(
                "17.1",
                "7, 7(b5) & 7(#5) - Ascending",
                vec!["8.4", "11.4", "12.4"],
            ),
            EarMasterLesson::new(
                "17.2",
                "7, 7(b5) & 7(#5) - Descending",
                vec!["8.4", "11.4", "12.4"],
            ),
            EarMasterLesson::new(
                "17.3",
                "7, 7(b5) & 7(#5) - Harmonic",
                vec!["8.4", "11.4", "12.4"],
            ),
            EarMasterLesson::new(
                "17.4",
                "7, 7(b5) & 7(#5) - Harmonic, Ascending & Descending",
                vec!["17.1", "17.2", "17.3"],
            ),
            // Unit 18.
            EarMasterLesson::new(
                "18.1",
                "Mi7, Mi,maj7, Dim7, Mi7(b5) & dim,maj7 - Ascending",
                vec!["9.4", "10.4"],
            ),
            EarMasterLesson::new(
                "18.2",
                "Mi7, Mi,maj7, Dim7, Mi7(b5) & dim,maj7 - Descending",
                vec!["9.4", "10.4"],
            ),
            EarMasterLesson::new(
                "18.3",
                "Mi7, Mi,maj7, Dim7, Mi7(b5) & dim,maj7 - Harmonic",
                vec!["9.4", "10.4"],
            ),
            EarMasterLesson::new(
                "18.4",
                "Mi7, Mi,maj7, Dim7, Mi7(b5) & dim,maj7 - Harmonic, Ascending & Descending",
                vec!["18.1", "18.2", "18.3"],
            ),
            // Unit 19.
            EarMasterLesson::new(
                "19.1",
                "All the Maj7 Chords - Ascending",
                vec!["15.4", "16.4", "18.4"],
            ),
            EarMasterLesson::new(
                "19.2",
                "All the Maj7 Chords - Descending",
                vec!["15.4", "16.4", "18.4"],
            ),
            EarMasterLesson::new(
                "19.3",
                "All the Maj7 Chords - Harmonic",
                vec!["15.4", "16.4", "18.4"],
            ),
            EarMasterLesson::new(
                "19.4",
                "All the Maj7 Chords - Harmonic, Ascending & Descending",
                vec!["19.1", "19.2", "19.3"],
            ),
            // Unit 20.
            EarMasterLesson::new(
                "20.1",
                "All chords with a minor 7th - Ascending",
                vec!["15.4", "17.4", "18.4"],
            ),
            EarMasterLesson::new(
                "20.2",
                "All chords with a minor 7th - Descending",
                vec!["15.4", "17.4", "18.4"],
            ),
            EarMasterLesson::new(
                "20.3",
                "All chords with a minor 7th - Harmonic",
                vec!["15.4", "17.4", "18.4"],
            ),
            EarMasterLesson::new(
                "20.4",
                "All chords with a minor 7th - Harmonic, Ascending & Descending",
                vec!["20.1", "20.2", "20.3"],
            ),
            // Unit 21.
            EarMasterLesson::new(
                "21.1",
                "Chords with a Major 6th or Diminished 7th - Ascending",
                vec!["8.4", "9.4", "10.4"],
            ),
            EarMasterLesson::new(
                "21.2",
                "Chords with a Major 6th or Diminished 7th - Descending",
                vec!["8.4", "9.4", "10.4"],
            ),
            EarMasterLesson::new(
                "21.3",
                "Chords with a Major 6th or Diminished 7th - Harmonic",
                vec!["8.4", "9.4", "10.4"],
            ),
            EarMasterLesson::new(
                "21.4",
                "Chords with a Major 6th or Diminished 7th - Harmonic, Ascending & Descending",
                vec!["21.1", "21.2", "21.3"],
            ),
            // Unit 22.
            EarMasterLesson::new(
                "22.1",
                "All 7th Chords - Ascending",
                vec!["19.4", "20.4", "21.4"],
            ),
            EarMasterLesson::new(
                "22.2",
                "All 7th Chords - Descending",
                vec!["19.4", "20.4", "21.4"],
            ),
            EarMasterLesson::new(
                "22.3",
                "All 7th Chords - Harmonic",
                vec!["19.4", "20.4", "21.4"],
            ),
            EarMasterLesson::new(
                "22.4",
                "All 7th Chords - Harmonic, Ascending & Descending",
                vec!["22.1", "22.2", "22.3"],
            ),
        ],
    };
    earmaster_course.course_builder()
}
