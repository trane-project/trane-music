use std::collections::BTreeMap;

use lazy_static::lazy_static;
use trane::course_builder::CourseBuilder;
use ustr::Ustr;

use super::{EarMasterCourse, EarMasterLesson};

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::music::earmaster::chord_inversions");
}

pub fn course_builder() -> CourseBuilder {
    let earmaster_course = EarMasterCourse {
        id: *COURSE_ID,
        name: "Chord Inversions".to_string(),
        directory_name: "earmaster_chord_inversions".to_string(),
        metadata: Some(BTreeMap::from([])),
        lessons: vec![
            // Unit 1 - Inversions of the major chord
            EarMasterLesson::new("1.1", "Inversions of the major chord - Ascending", vec![]),
            EarMasterLesson::new(
                "1.2",
                "Inversions of the major chord - Descending",
                vec!["1.1"],
            ),
            EarMasterLesson::new(
                "1.3",
                "Inversions of the major chord - Harmonic",
                vec!["1.2"],
            ),
            EarMasterLesson::new(
                "1.4",
                "Inversions of the major chord - Harmonic, ascending and descending",
                vec!["1.3"],
            ),
            // Unit 2 - Inversions of the minor chord
            EarMasterLesson::new(
                "2.1",
                "Inversions of the minor chord - Ascending",
                vec!["1.4"],
            ),
            EarMasterLesson::new(
                "2.2",
                "Inversions of the minor chord - Descending",
                vec!["2.1"],
            ),
            EarMasterLesson::new(
                "2.3",
                "Inversions of the minor chord - Harmonic",
                vec!["2.2"],
            ),
            EarMasterLesson::new(
                "2.4",
                "Inversions of the minor chord - Harmonic, ascending and descending",
                vec!["2.3"],
            ),
            // Unit 3 - Inversions of the sus4 chord
            EarMasterLesson::new(
                "3.1",
                "Inversions of the sus4 chord - Ascending",
                vec!["2.4"],
            ),
            EarMasterLesson::new(
                "3.2",
                "Inversions of the sus4 chord - Descending",
                vec!["3.1"],
            ),
            EarMasterLesson::new(
                "3.3",
                "Inversions of the sus4 chord - Harmonic",
                vec!["3.2"],
            ),
            EarMasterLesson::new(
                "3.4",
                "Inversions of the sus4 chord - Harmonic, ascending and descending",
                vec!["3.3"],
            ),
            // Unit 4 - Inversions of the major(b5) chord
            EarMasterLesson::new(
                "4.1",
                "Inversions of the major(b5) chord - Ascending",
                vec!["3.4"],
            ),
            EarMasterLesson::new(
                "4.2",
                "Inversions of the major(b5) chord - Descending",
                vec!["4.1"],
            ),
            EarMasterLesson::new(
                "4.3",
                "Inversions of the major(b5) chord - Harmonic",
                vec!["4.2"],
            ),
            EarMasterLesson::new(
                "4.4",
                "Inversions of the major(b5) chord - Harmonic, ascending and descending",
                vec!["4.3"],
            ),
            // Unit 5 - Inversions of the dim chord
            EarMasterLesson::new(
                "5.1",
                "Inversions of the dim chord - Ascending",
                vec!["4.4"],
            ),
            EarMasterLesson::new(
                "5.2",
                "Inversions of the dim chord - Descending",
                vec!["5.1"],
            ),
            EarMasterLesson::new("5.3", "Inversions of the dim chord - Harmonic", vec!["5.2"]),
            EarMasterLesson::new(
                "5.4",
                "Inversions of the dim chord - Harmonic, ascending and descending",
                vec!["5.3"],
            ),
            // Unit 6 - Major with perfect or altered 5th
            EarMasterLesson::new("6.1", "Major, aug & major(b5) - Ascending", vec!["5.4"]),
            EarMasterLesson::new("6.2", "Major, aug & major(b5) - Descending", vec!["6.1"]),
            EarMasterLesson::new("6.3", "Major, aug & major(b5) - Harmonic", vec!["6.2"]),
            EarMasterLesson::new(
                "6.4",
                "Major, aug & major(b5) - Harmonic, ascending and descending",
                vec!["6.3"],
            ),
            // Unit 7 - Minor with perfect or altered 5th
            EarMasterLesson::new(
                "7.1",
                "Inversions of the minor & dim chords - Ascending",
                vec!["6.4"],
            ),
            EarMasterLesson::new(
                "7.2",
                "Inversions of the minor & dim chords - Descending",
                vec!["7.1"],
            ),
            EarMasterLesson::new(
                "7.3",
                "Inversions of the minor & dim chords - Harmonic",
                vec!["7.2"],
            ),
            EarMasterLesson::new(
                "7.4",
                "Inversions of the minor & dim chords - Harmonic, ascending and descending",
                vec!["7.3"],
            ),
            // Unit 8 - Minor & major with altered 5th
            EarMasterLesson::new(
                "8.1",
                "Inversions of the Dim, aug & major(b5) chords - Ascending",
                vec!["7.4"],
            ),
            EarMasterLesson::new(
                "8.2",
                "Inversions of the Dim, aug & major(b5) chords - Descending",
                vec!["8.1"],
            ),
            EarMasterLesson::new(
                "8.3",
                "Inversions of the Dim, aug & major(b5) chords - Harmonic",
                vec!["8.2"],
            ),
            EarMasterLesson::new(
                "8.4",
                "Inversions of the Dim, aug & major(b5) chords - Harmonic, ascending and \
                 descending",
                vec!["8.3"],
            ),
            // Unit 9 - All the triads
            EarMasterLesson::new(
                "9.1",
                "Major, minor, dim, aug, sus4 & major(b5) - Ascending",
                vec!["8.4"],
            ),
            EarMasterLesson::new(
                "9.2",
                "Major, minor, dim, aug, sus4 & major(b5) - Descending",
                vec!["9.1"],
            ),
            EarMasterLesson::new(
                "9.3",
                "Major, minor, dim, aug, sus4 & major(b5) - Harmonic",
                vec!["9.2"],
            ),
            EarMasterLesson::new(
                "9.4",
                "Major, minor, dim, aug, sus4 & major(b5) - Harmonic, ascending and descending",
                vec!["9.3"],
            ),
            // Unit 10 - Inversions of the maj7 & 7 chords
            EarMasterLesson::new(
                "10.1",
                "Inversions of the maj7 & 7 chords - Ascending",
                vec!["9.4"],
            ),
            EarMasterLesson::new(
                "10.2",
                "Inversions of the maj7 & 7 chords - Descending",
                vec!["10.1"],
            ),
            EarMasterLesson::new(
                "10.3",
                "Inversions of the maj7 & 7 chords - Harmonic",
                vec!["10.2"],
            ),
            EarMasterLesson::new(
                "10.4",
                "Inversions of the maj7 & 7 chords - Harmonic, ascending and descending",
                vec!["10.3"],
            ),
            // Unit 11 - Inversions of the mi7 & mi,maj7 chords
            EarMasterLesson::new(
                "11.1",
                "Inversions of the mi7 & mi,maj7 chords - Ascending",
                vec!["10.4"],
            ),
            EarMasterLesson::new(
                "11.2",
                "Inversions of the mi7 & mi,maj7 chords - Descending",
                vec!["11.1"],
            ),
            EarMasterLesson::new(
                "11.3",
                "Inversions of the mi7 & mi,maj7 chords - Harmonic",
                vec!["11.2"],
            ),
            EarMasterLesson::new(
                "11.4",
                "Inversions of the mi7 & mi,maj7 chords - Harmonic, ascending and descending",
                vec!["11.3"],
            ),
            // Unit 12 - Mi7(b5), dim7 & dim,maj7
            EarMasterLesson::new("12.1", "Mi7(b5), dim7 & dim,maj7 - Ascending", vec!["11.4"]),
            EarMasterLesson::new(
                "12.2",
                "Mi7(b5), dim7 & dim,maj7 - Descending",
                vec!["12.1"],
            ),
            EarMasterLesson::new("12.3", "Mi7(b5), dim7 & dim,maj7 - Harmonic", vec!["12.2"]),
            EarMasterLesson::new(
                "12.4",
                "Mi7(b5), dim7 & dim,maj7 - Harmonic, ascending and descending",
                vec!["12.3"],
            ),
            // Unit 13 - Inversions of the 7(b5) & maj7(b5) chords
            EarMasterLesson::new(
                "13.1",
                "Inversions of the 7(b5) & maj7(b5) chords - Ascending",
                vec!["12.4"],
            ),
            EarMasterLesson::new(
                "13.2",
                "Inversions of the 7(b5) & maj7(b5) chords - Descending",
                vec!["13.1"],
            ),
            EarMasterLesson::new(
                "13.3",
                "Inversions of the 7(b5) & maj7(b5) chords - Harmonic",
                vec!["13.2"],
            ),
            EarMasterLesson::new(
                "13.4",
                "Inversions of the 7(b5) & maj7(b5) chords - Harmonic, ascending and descending",
                vec!["13.3"],
            ),
            // Unit 14 - Inversions of the 7(#5) & maj7(#5) chords
            EarMasterLesson::new(
                "14.1",
                "Inversions of the 7(#5) & maj7(#5) chords - Ascending",
                vec!["13.4"],
            ),
            EarMasterLesson::new(
                "14.2",
                "Inversions of the 7(#5) & maj7(#5) chords - Descending",
                vec!["14.1"],
            ),
            EarMasterLesson::new(
                "14.3",
                "Inversions of the 7(#5) & maj7(#5) chords - Harmonic",
                vec!["14.2"],
            ),
            EarMasterLesson::new(
                "14.4",
                "Inversions of the 7(#5) & maj7(#5) chords - Harmonic, ascending and descending",
                vec!["14.3"],
            ),
            // Unit 15 - Inversions of the 7, 7(b5) & 7(#5) chords
            EarMasterLesson::new(
                "15.1",
                "Inversions of the 7, 7(b5) & 7(#5) chords - Ascending",
                vec!["14.4"],
            ),
            EarMasterLesson::new(
                "15.2",
                "Inversions of the 7, 7(b5) & 7(#5) chords - Descending",
                vec!["15.1"],
            ),
            EarMasterLesson::new(
                "15.3",
                "Inversions of the 7, 7(b5) & 7(#5) chords - Harmonic",
                vec!["15.2"],
            ),
            EarMasterLesson::new(
                "15.4",
                "Inversions of the 7, 7(b5) & 7(#5) chords - Harmonic, ascending and descending",
                vec!["15.3"],
            ),
            // Unit 16 - Maj7, maj7(b5) & maj7(#5)
            EarMasterLesson::new(
                "16.1",
                "Maj7, maj7(b5) & maj7(#5) - Ascending",
                vec!["15.4"],
            ),
            EarMasterLesson::new(
                "16.2",
                "Maj7, maj7(b5) & maj7(#5) - Descending",
                vec!["16.1"],
            ),
            EarMasterLesson::new("16.3", "Maj7, maj7(b5) & maj7(#5) - Harmonic", vec!["16.2"]),
            EarMasterLesson::new(
                "16.4",
                "Maj7, maj7(b5) & maj7(#5) - Harmonic, ascending and descending",
                vec!["16.3"],
            ),
            // Unit 17 - Mi7, mi7(b5), dim7, dim,maj7 & mi,maj7
            EarMasterLesson::new(
                "17.1",
                "Mi7, mi7(b5), dim7, dim,maj7 & mi,maj7 - Ascending",
                vec!["16.4"],
            ),
            EarMasterLesson::new(
                "17.2",
                "Mi7, mi7(b5), dim7, dim,maj7 & mi,maj7 - Descending",
                vec!["17.1"],
            ),
            EarMasterLesson::new(
                "17.3",
                "Mi7, mi7(b5), dim7, dim,maj7 & mi,maj7 - Harmonic",
                vec!["17.2"],
            ),
            EarMasterLesson::new(
                "17.4",
                "Mi7, mi7(b5), dim7, dim,maj7 & mi,maj7 - Harmonic, ascending and descending",
                vec!["17.3"],
            ),
            // Unit 18 - All the 7 chords
            EarMasterLesson::new(
                "18.1",
                "Maj7, 7, mi7, mi7(b5), dim7, dim,maj7, 7(b5), maj7(b5), 7(#5), maj7(#5) & \
                mi,maj7 - Ascending",
                vec!["17.4"],
            ),
            EarMasterLesson::new(
                "18.2",
                "Maj7, 7, mi7, mi7(b5), dim7, dim,maj7, 7(b5), maj7(b5), 7(#5), maj7(#5) & \
                mi,maj7 - Descending",
                vec!["18.1"],
            ),
            EarMasterLesson::new(
                "18.3",
                "Maj7, 7, mi7, mi7(b5), dim7, dim,maj7, 7(b5), maj7(b5), 7(#5), maj7(#5) & \
                mi,maj7 - Harmonic",
                vec!["18.2"],
            ),
            EarMasterLesson::new(
                "18.4",
                "Maj7, 7, mi7, mi7(b5), dim7, dim,maj7, 7(b5), maj7(b5), 7(#5), maj7(#5) & \
                mi,maj7 - Harmonic, ascending and descending",
                vec!["18.3"],
            ),
        ],
    };
    earmaster_course.course_builder()
}
