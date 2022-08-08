use std::collections::BTreeMap;

use lazy_static::lazy_static;
use trane::course_builder::CourseBuilder;
use ustr::Ustr;

use super::{EarMasterCourse, EarMasterLesson};

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::music::earmaster::rhthym_sight_reading");
}

pub fn course_builder() -> CourseBuilder {
    let earmaster_course = EarMasterCourse {
        id: *COURSE_ID,
        name: "Rhythm Sight-Reading".to_string(),
        directory_name: "earmaster_rhythm_sight_reading".to_string(),
        metadata: Some(BTreeMap::from([])),
        lessons: vec![
            // Unit 1 - 4/4 - 4 bars: Whole, Half, Quarter & Eighth notes
            EarMasterLesson::new(
                "1.1",
                "4/4 - 4 bars: Quarter notes - including rests",
                vec![],
            ),
            EarMasterLesson::new("1.2", "4/4 - 4 bars: 8th & Quarter notes", vec!["1.1"]),
            EarMasterLesson::new(
                "1.3",
                "4/4 - 4 bars: 8th notes - including rests",
                vec!["1.2"],
            ),
            EarMasterLesson::new(
                "1.4",
                "4/4 - 4 bars: 8th & Quarter notes - including rests",
                vec!["1.3"],
            ),
            EarMasterLesson::new(
                "1.5",
                "4/4 - 4 bars: Quarter & Half Notes - including rests",
                vec!["1.4"],
            ),
            EarMasterLesson::new(
                "1.6",
                "4/4 - 4 bars: Half notes & Whole notes - including rests",
                vec!["1.5"],
            ),
            EarMasterLesson::new(
                "1.7",
                "4/4 - 4 bars: Quarter, Half & Whole notes - including rests",
                vec!["1.6"],
            ),
            EarMasterLesson::new(
                "1.8",
                "4/4 - 4 bars: 8th, Quarter, Half & Whole notes - including rests",
                vec!["1.7"],
            ),
            // Unit 2 - 3/4 - 4 bars: Half, Quarter & Eighth notes
            EarMasterLesson::new(
                "2.1",
                "3/4 - 4 bars: Quarter notes - including rests",
                vec![],
            ),
            EarMasterLesson::new("2.2", "3/4 - 4 bars: 8th & Quarter notes", vec!["2.1"]),
            EarMasterLesson::new(
                "2.3",
                "3/4 - 4 bars: 8th notes - including rests",
                vec!["2.2"],
            ),
            EarMasterLesson::new(
                "2.4",
                "3/4 - 4 bars: 8th & Quarter notes - including rests",
                vec!["2.3"],
            ),
            EarMasterLesson::new(
                "2.5",
                "3/4 - 4 bars: Quarter & Half Notes - including rests",
                vec!["2.4"],
            ),
            EarMasterLesson::new(
                "2.6",
                "3/4 - 4 bars: 8th, Quarter & Half notes - including rests",
                vec!["2.5"],
            ),
            // Unit 3 - 2/4 - 4 bars: Half, Quarter & Eighth notes
            EarMasterLesson::new(
                "3.1",
                "2/4 - 4 bars: Quarter notes - including rests",
                vec![],
            ),
            EarMasterLesson::new("3.2", "2/4 - 4 bars: 8th & Quarter notes", vec!["3.1"]),
            EarMasterLesson::new(
                "3.3",
                "2/4 - 4 bars: 8th notes - including rests",
                vec!["3.2"],
            ),
            EarMasterLesson::new(
                "3.4",
                "2/4 - 4 bars: 8th & Quarter notes - including rests",
                vec!["3.3"],
            ),
            EarMasterLesson::new(
                "3.5",
                "2/4 - 4 bars: Quarter & Half Notes - including rests",
                vec!["3.4"],
            ),
            EarMasterLesson::new(
                "3.6",
                "2/4 - 4 bars: 8th, Quarter & Half notes - including rests",
                vec!["3.5"],
            ),
            // Unit 4 - 4/4 - 8 bars: Whole, Half, Quarter & Eighth notes
            EarMasterLesson::new(
                "4.1",
                "4/4 - 8 bars: Quarter notes - including rests",
                vec!["1.8"],
            ),
            EarMasterLesson::new("4.2", "4/4 - 8 bars: 8th & Quarter notes", vec!["4.1"]),
            EarMasterLesson::new(
                "4.3",
                "4/4 - 8 bars: 8th notes - including rests",
                vec!["4.2"],
            ),
            EarMasterLesson::new(
                "4.4",
                "4/4 - 8 bars: 8th & Quarter notes - including rests",
                vec!["4.3"],
            ),
            EarMasterLesson::new(
                "4.5",
                "4/4 - 8 bars: Quarter & Half Notes - including rests",
                vec!["4.4"],
            ),
            EarMasterLesson::new(
                "4.6",
                "4/4 - 8 bars: Half notes & Whole notes - including rests",
                vec!["4.5"],
            ),
            EarMasterLesson::new(
                "4.7",
                "4/4 - 8 bars: Quarter, Half & Whole notes - including rests",
                vec!["4.6"],
            ),
            EarMasterLesson::new(
                "4.8",
                "4/4 - 8 bars: 8th, Quarter, Half & Whole notes - including rests",
                vec!["4.7"],
            ),
            // Unit 5 - 3/4 - 8 bars: Half, Quarter & Eighth notes
            EarMasterLesson::new(
                "5.1",
                "3/4 - 8 bars: Quarter notes - including rests",
                vec!["2.6"],
            ),
            EarMasterLesson::new("5.2", "3/4 - 8 bars: 8th & Quarter notes", vec!["5.1"]),
            EarMasterLesson::new(
                "5.3",
                "3/4 - 8 bars: 8th notes - including rests",
                vec!["5.2"],
            ),
            EarMasterLesson::new(
                "5.4",
                "3/4 - 8 bars: 8th & Quarter notes - including rests",
                vec!["5.3"],
            ),
            EarMasterLesson::new(
                "5.5",
                "3/4 - 8 bars: Quarter & Half Notes - including rests",
                vec!["5.4"],
            ),
            EarMasterLesson::new(
                "5.6",
                "3/4 - 8 bars: 8th, Quarter & Half notes - including rests",
                vec!["5.5"],
            ),
            // Unit 6 - 2/4 - 8 bars: Half, Quarter & Eighth notes
            EarMasterLesson::new(
                "6.1",
                "2/4 - 8 bars: Quarter notes - including rests",
                vec!["3.6"],
            ),
            EarMasterLesson::new("6.2", "2/4 - 8 bars: 8th & Quarter notes", vec!["6.1"]),
            EarMasterLesson::new(
                "6.3",
                "2/4 - 8 bars: 8th notes - including rests",
                vec!["6.2"],
            ),
            EarMasterLesson::new(
                "6.4",
                "2/4 - 8 bars: 8th & Quarter notes - including rests",
                vec!["6.3"],
            ),
            EarMasterLesson::new(
                "6.5",
                "2/4 - 8 bars: Quarter & Half Notes - including rests",
                vec!["6.4"],
            ),
            EarMasterLesson::new(
                "6.6",
                "2/4 - 8 bars: 8th, Quarter & Half notes - including rests",
                vec!["6.5"],
            ),
            // Unit 7 - 4/4: Introducing Sixteenth Note Groupings
            EarMasterLesson::new(
                "7.1",
                "4/4 - 4 bars: 16th & Quarter notes - including rests",
                vec!["4.8"],
            ),
            EarMasterLesson::new(
                "7.2",
                "4/4 - 4 bars: 16th, 8th & Quarter notes - including rests",
                vec!["7.1"],
            ),
            EarMasterLesson::new(
                "7.3",
                "4/4 - 4 bars: 16th, 8th & Quarter notes - including rests",
                vec!["7.2"],
            ),
            // Unit 8 - 3/4: Introducing Sixteenth Note Groupings
            EarMasterLesson::new(
                "8.1",
                "3/4 - 4 bars: 16th & Quarter notes - including rests",
                vec!["5.6"],
            ),
            EarMasterLesson::new(
                "8.2",
                "3/4 - 4 bars: 16th, 8th & Quarter notes - including rests",
                vec!["8.1"],
            ),
            EarMasterLesson::new(
                "8.3",
                "3/4 - 4 bars: 16th, 8th & Quarter notes - including rests",
                vec!["8.2"],
            ), // Unit 9 - 2/4: Introducing Sixteenth Note Groupings
            EarMasterLesson::new(
                "9.1",
                "2/4 - 4 bars: 16th & Quarter notes - including rests",
                vec!["6.6"],
            ),
            EarMasterLesson::new(
                "9.2",
                "2/4 - 4 bars: 16th, 8th & Quarter notes - including rests",
                vec!["9.1"],
            ),
            EarMasterLesson::new(
                "9.3",
                "2/4 - 4 bars: 16th, 8th & Quarter notes - including rests",
                vec!["9.2"],
            ),
        ],
    };
    earmaster_course.course_builder()
}
