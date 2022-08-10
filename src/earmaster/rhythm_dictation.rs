use std::collections::BTreeMap;

use lazy_static::lazy_static;
use trane::course_builder::CourseBuilder;
use ustr::Ustr;

use super::{EarMasterCourse, EarMasterLesson};

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::music::earmaster::rhythm_dictation");
}

pub fn course_builder() -> CourseBuilder {
    let earmaster_course = EarMasterCourse {
        id: *COURSE_ID,
        name: "Rhythm Dictation".to_string(),
        directory_name: "earmaster_rhythm_dictation".to_string(),
        metadata: Some(BTreeMap::from([])),
        lessons: vec![
            // Unit 1 - 4/4 - 1 bar: Whole, Half, Quarter & Eighth notes
            EarMasterLesson::new(
                "1.1",
                "4/4 - 1 bar: Quarter notes - including rests",
                vec![],
            ),
            EarMasterLesson::new("1.2", "4/4 - 1 bar: 8th & Quarter notes", vec!["1.1"]),
            EarMasterLesson::new(
                "1.3",
                "4/4 - 1 bar: 8th notes - including rests",
                vec!["1.2"],
            ),
            EarMasterLesson::new(
                "1.4",
                "4/4 - 1 bar: 8th & Quarter notes - including rests",
                vec!["1.3"],
            ),
            EarMasterLesson::new(
                "1.5",
                "4/4 - 1 bar: Quarter & Half Notes - including rests",
                vec!["1.4"],
            ),
            EarMasterLesson::new(
                "1.6",
                "4/4 - 1 bar: Half notes & Whole notes - including rests",
                vec!["1.5"],
            ),
            EarMasterLesson::new(
                "1.7",
                "4/4 - 1 bar: Quarter, Half & Whole notes - including rests",
                vec!["1.6"],
            ),
            EarMasterLesson::new(
                "1.8",
                "4/4 - 1 bar: 8th, Quarter, Half & Whole notes - including rests",
                vec!["1.7"],
            ),
            // Unit 2 - 3/4 - 1 bar: Half, Quarter & Eighth notes
            EarMasterLesson::new(
                "2.1",
                "3/4 - 1 bar: Quarter notes - including rests",
                vec![],
            ),
            EarMasterLesson::new("2.2", "3/4 - 1 bar: 8th & Quarter notes", vec!["2.1"]),
            EarMasterLesson::new(
                "2.3",
                "3/4 - 1 bar: 8th notes - including rests",
                vec!["2.2"],
            ),
            EarMasterLesson::new(
                "2.4",
                "3/4 - 1 bar: 8th & Quarter notes - including rests",
                vec!["2.3"],
            ),
            EarMasterLesson::new(
                "2.5",
                "3/4 - 1 bar: Quarter & Half Notes - including rests",
                vec!["2.4"],
            ),
            EarMasterLesson::new(
                "2.6",
                "3/4 - 1 bar: 8th, Quarter & Half notes - including rests",
                vec!["2.5"],
            ),
            // Unit 3 - 2/4 - 1 bar: Half, Quarter & Eighth notes
            EarMasterLesson::new(
                "3.1",
                "2/4 - 1 bar: Quarter notes - including rests",
                vec![],
            ),
            EarMasterLesson::new("3.2", "2/4 - 1 bar: 8th & Quarter notes", vec!["3.1"]),
            EarMasterLesson::new(
                "3.3",
                "2/4 - 1 bar: 8th notes - including rests",
                vec!["3.2"],
            ),
            EarMasterLesson::new(
                "3.4",
                "2/4 - 1 bar: 8th & Quarter notes - including rests",
                vec!["3.3"],
            ),
            EarMasterLesson::new(
                "3.5",
                "2/4 - 1 bar: Quarter & Half Notes - including rests",
                vec!["3.4"],
            ),
            EarMasterLesson::new(
                "3.6",
                "2/4 - 1 bar: 8th, Quarter & Half notes - including rests",
                vec!["3.5"],
            ),
            // Unit 4 - 4/4 - 2 bars: Whole, Half, Quarter & Eighth notes
            EarMasterLesson::new(
                "4.1",
                "4/4 - 2 bars: Quarter notes - including rests",
                vec!["1.8"],
            ),
            EarMasterLesson::new("4.2", "4/4 - 2 bars: 8th & Quarter notes", vec!["4.1"]),
            EarMasterLesson::new(
                "4.3",
                "4/4 - 2 bars: 8th notes - including rests",
                vec!["4.2"],
            ),
            EarMasterLesson::new(
                "4.4",
                "4/4 - 2 bars: 8th & Quarter notes - including rests",
                vec!["4.3"],
            ),
            EarMasterLesson::new(
                "4.5",
                "4/4 - 2 bars: Quarter & Half Notes - including rests",
                vec!["4.4"],
            ),
            EarMasterLesson::new(
                "4.6",
                "4/4 - 2 bars: Half notes & Whole notes - including rests",
                vec!["4.5"],
            ),
            EarMasterLesson::new(
                "4.7",
                "4/4 - 2 bars: Quarter, Half & Whole notes - including rests",
                vec!["4.6"],
            ),
            EarMasterLesson::new(
                "4.8",
                "4/4 - 2 bars: 8th, Quarter, Half & Whole notes - including rests",
                vec!["4.7"],
            ),
            // Unit 5 - 3/4 - 2 bars: Half, Quarter & Eighth notes
            EarMasterLesson::new(
                "5.1",
                "3/4 - 2 bars: Quarter notes - including rests",
                vec!["2.6"],
            ),
            EarMasterLesson::new("5.2", "3/4 - 2 bars: 8th & Quarter notes", vec!["5.1"]),
            EarMasterLesson::new(
                "5.3",
                "3/4 - 2 bars: 8th notes - including rests",
                vec!["5.2"],
            ),
            EarMasterLesson::new(
                "5.4",
                "3/4 - 2 bars: 8th & Quarter notes - including rests",
                vec!["5.3"],
            ),
            EarMasterLesson::new(
                "5.5",
                "3/4 - 2 bars: Quarter & Half Notes - including rests",
                vec!["5.4"],
            ),
            EarMasterLesson::new(
                "5.6",
                "3/4 - 2 bars: 8th, Quarter & Half notes - including rests",
                vec!["5.5"],
            ),
            // Unit 6 - 2/4 - 2 bars: Half, Quarter & Eighth notes
            EarMasterLesson::new(
                "6.1",
                "2/4 - 2 bars: Quarter notes - including rests",
                vec!["3.6"],
            ),
            EarMasterLesson::new("6.2", "2/4 - 2 bars: 8th & Quarter notes", vec!["6.1"]),
            EarMasterLesson::new(
                "6.3",
                "2/4 - 2 bars: 8th notes - including rests",
                vec!["6.2"],
            ),
            EarMasterLesson::new(
                "6.4",
                "2/4 - 2 bars: 8th & Quarter notes - including rests",
                vec!["6.3"],
            ),
            EarMasterLesson::new(
                "6.5",
                "2/4 - 2 bars: Quarter & Half Notes - including rests",
                vec!["6.4"],
            ),
            EarMasterLesson::new(
                "6.6",
                "2/4 - 2 bars: 8th, Quarter & Half notes - including rests",
                vec!["6.5"],
            ),
            // Unit 7 - 4/4: Introducing Sixteenth Note Groupings
            EarMasterLesson::new(
                "7.1",
                "4/4 - 1 bar: 16th & Quarter notes - including rests",
                vec!["4.8"],
            ),
            EarMasterLesson::new(
                "7.2",
                "4/4 - 1 bar: 16th, 8th & Quarter notes - including rests",
                vec!["7.1"],
            ),
            EarMasterLesson::new(
                "7.3",
                "4/4 - 1 bar: 16th, 8th & Quarter notes - including rests",
                vec!["7.2"],
            ),
            // Unit 8 - 3/4: Introducing Sixteenth Note Groupings
            EarMasterLesson::new(
                "8.1",
                "3/4 - 1 bar: 16th & Quarter notes - including rests",
                vec!["5.6"],
            ),
            EarMasterLesson::new(
                "8.2",
                "3/4 - 1 bar: 16th, 8th & Quarter notes - including rests",
                vec!["8.1"],
            ),
            EarMasterLesson::new(
                "8.3",
                "3/4 - 1 bar: 16th, 8th & Quarter notes - including rests",
                vec!["8.2"],
            ),
            // Unit 9 - 2/4: Introducing Sixteenth Note Groupings
            EarMasterLesson::new(
                "9.1",
                "2/4 - 1 bar: 16th & Quarter notes - including rests",
                vec!["6.6"],
            ),
            EarMasterLesson::new(
                "9.2",
                "2/4 - 1 bar: 16th, 8th & Quarter notes - including rests",
                vec!["9.1"],
            ),
            EarMasterLesson::new(
                "9.3",
                "2/4 - 1 bar: 16th, 8th & Quarter notes - including rests",
                vec!["9.2"],
            ),
            // Unit 10 - Advanced Sixteenth Note Groupings
            EarMasterLesson::new(
                "10.1",
                "4/4 - 1 bar: 16th, 8th & Quarter notes",
                vec!["7.3"],
            ),
            EarMasterLesson::new(
                "10.2",
                "3/4 - 1 bar: 16th, 8th & Quarter notes",
                vec!["8.3"],
            ),
            EarMasterLesson::new(
                "10.3",
                "2/4 - 1 bar: 16th, 8th & Quarter notes",
                vec!["9.3"],
            ),
            EarMasterLesson::new(
                "10.4",
                "2/4, 3/4 & 4/4 - 1 bar: 16th, 8th & Quarter notes",
                vec!["10.1", "10.2", "10.3"],
            ),
            EarMasterLesson::new(
                "10.5",
                "4/4 - 1 bar: 16th, 8th & Quarter notes",
                vec!["10.4"],
            ),
            EarMasterLesson::new(
                "10.6",
                "3/4 - 1 bar: 16th, 8th & Quarter notes",
                vec!["10.4"],
            ),
            EarMasterLesson::new(
                "10.7",
                "2/4 - 1 bar: 16th, 8th & Quarter notes",
                vec!["10.4"],
            ),
            EarMasterLesson::new(
                "10.8",
                "2/4, 3/4 & 4/4 - 1 bar: 16th, 8th & Quarter notes",
                vec!["10.5", "10.6", "10.7"],
            ),
            // Unit 11 - Advanced Sixteenth Note Groupings with Rests
            EarMasterLesson::new(
                "11.1",
                "4/4 - 1 bar: 16th, 8th & Quarter notes - including rests",
                vec!["10.8"],
            ),
            EarMasterLesson::new(
                "11.2",
                "3/4 - 1 bar: 16th, 8th & Quarter notes - including rests",
                vec!["10.8"],
            ),
            EarMasterLesson::new(
                "11.3",
                "2/4 - 1 bar: 16th, 8th & Quarter notes - including rests",
                vec!["10.8"],
            ),
            EarMasterLesson::new(
                "11.4",
                "2/4, 3/4 & 4/4 - 1 bar: 16th, 8th & Quarter notes - including rests",
                vec!["11.1", "11.2", "11.3"],
            ),
            EarMasterLesson::new(
                "11.5",
                "4/4 - 1 bar: 16th, 8th & Quarter notes - including rests",
                vec!["11.4"],
            ),
            EarMasterLesson::new(
                "11.6",
                "3/4 - 1 bar: 16th, 8th & Quarter notes - including rests",
                vec!["11.4"],
            ),
            EarMasterLesson::new(
                "11.7",
                "2/4 - 1 bar: 16th, 8th & Quarter notes - including rests",
                vec!["11.4"],
            ),
            EarMasterLesson::new(
                "11.8",
                "2/4, 3/4 & 4/4 - 1 bar: 16th, 8th & Quarter notes - including rests",
                vec!["11.5", "11.6", "11.7"],
            ),
            // Unit 12 - Advanced 16th and 8th Combinations with Rests
            EarMasterLesson::new(
                "12.1",
                "4/4 - 1 bar: 16th, 8th & Quarter notes - including rests",
                vec!["11.8"],
            ),
            EarMasterLesson::new(
                "12.2",
                "3/4 - 1 bar: 16th, 8th & Quarter notes - including rests",
                vec!["11.8"],
            ),
            EarMasterLesson::new(
                "12.3",
                "2/4 - 1 bar: 16th, 8th & Quarter notes - including rests",
                vec!["11.8"],
            ),
            EarMasterLesson::new(
                "12.4",
                "2/4, 3/4 & 4/4 - 1 bar: 16th, 8th & Quarter notes - including rests",
                vec!["12.1", "12.2", "12.3"],
            ),
            EarMasterLesson::new(
                "12.5",
                "4/4 - 1 bar: 16th, 8th & Quarter notes - including rests",
                vec!["12.4"],
            ),
            EarMasterLesson::new(
                "12.6",
                "3/4 - 1 bar: 16th, 8th & Quarter notes - including rests",
                vec!["12.4"],
            ),
            EarMasterLesson::new(
                "12.7",
                "2/4 - 1 bar: 16th, 8th & Quarter notes - including rests",
                vec!["12.4"],
            ),
            EarMasterLesson::new(
                "12.8",
                "2/4, 3/4 & 4/4 - 1 bar: 16th, 8th & Quarter notes - including rests",
                vec!["12.5", "12.6", "12.7"],
            ),
            // Unit 13 - All note values from the previous lessons, including rests
            EarMasterLesson::new(
                "13.1",
                "4/4 - 1 bar: 16th, 8th, Quarter, Half & Whole notes - including rests",
                vec!["12.8"],
            ),
            EarMasterLesson::new(
                "13.2",
                "3/4 - 1 bar: 16th, 8th, Quarter & Half notes - including rests",
                vec!["12.8"],
            ),
            EarMasterLesson::new(
                "13.3",
                "2/4 - 1 bar: 16th, 8th, Quarter & Half notes - including rests",
                vec!["12.8"],
            ),
            EarMasterLesson::new(
                "13.4",
                "2/4, 3/4 & 4/4 - 1 bar: 16th, 8th, Quarter & Half notes - including rests",
                vec!["13.1", "13.2", "13.3"],
            ),
            // Unit 14 - 4/4: Triplets
            EarMasterLesson::new(
                "14.1",
                "4/4 - 1 bar: Quarter notes, 8th Triplets",
                vec!["13.4"],
            ),
            EarMasterLesson::new(
                "14.2",
                "4/4 - 1 bar: Quarter notes, 8th Triplets - including rests",
                vec!["14.1"],
            ),
            EarMasterLesson::new(
                "14.3",
                "4/4 - 1 bar: Quarter & Half notes, 8th Triplets - including rests",
                vec!["14.2"],
            ),
            EarMasterLesson::new(
                "14.4",
                "4/4 - 1 bar: Quarter notes, 8th Triplets - including rests",
                vec!["14.3"],
            ),
            EarMasterLesson::new("14.5", "4/4 - 1 bar: 8th & Quarter Triplets", vec!["14.4"]),
            EarMasterLesson::new(
                "14.6",
                "4/4 - 1 bar: 8th & Quarter Triplets - including rests",
                vec!["14.5"],
            ),
            EarMasterLesson::new(
                "14.7",
                "4/4 - 1 bar: Quarter notes, 8th & Quarter Triplets",
                vec!["14.6"],
            ),
            EarMasterLesson::new(
                "14.8",
                "4/4 - 1 bar: Quarter notes, 8th & Quarter Triplets - including rests",
                vec!["14.7"],
            ),
            EarMasterLesson::new("14.9", "4/4 - 1 bar: Half triplets", vec!["14.8"]),
            EarMasterLesson::new(
                "14.10",
                "4/4 - 1 bar: Half triplets - including rests",
                vec!["14.9"],
            ),
            EarMasterLesson::new(
                "14.11",
                "4/4 - 1 bar: Quarter notes, 16th Triplets",
                vec!["14.10"],
            ),
            EarMasterLesson::new(
                "14.12",
                "4/4 - 1 bar: Quarter notes, 16th Triplets - including rests",
                vec!["14.11"],
            ),
            EarMasterLesson::new(
                "14.13",
                "4/4 - 1 bar: 16th Triplets - including rests",
                vec!["14.12"],
            ),
            EarMasterLesson::new("14.14", "4/4 - 1 bar: 16th & 8th Triplets", vec!["14.13"]),
            EarMasterLesson::new(
                "14.15",
                "4/4 - 1 bar: 16th & 8th Triplets - including rests",
                vec!["14.14"],
            ),
            EarMasterLesson::new(
                "14.16",
                "4/4 - 1 bar: 16th, 8th, Quarter & Half Triplets",
                vec!["14.15"],
            ),
            EarMasterLesson::new(
                "14.17",
                "4/4 - 1 bar: 16th, 8th, Quarter & Half Triplets - including rests",
                vec!["14.16"],
            ),
            // Unit 15 - Triplets, now in 3/4
            EarMasterLesson::new(
                "15.1",
                "3/4 - 1 bar: Quarter notes, 8th Triplets",
                vec!["13.4"],
            ),
            EarMasterLesson::new(
                "15.2",
                "3/4 - 1 bar: Quarter notes, 8th Triplets - including rests",
                vec!["15.1"],
            ),
            EarMasterLesson::new(
                "15.3",
                "3/4 - 1 bar: Quarter & Half notes, 8th Triplets - including rests",
                vec!["15.2"],
            ),
            EarMasterLesson::new(
                "15.4",
                "3/4 - 1 bar: Quarter notes, 8th Triplets - including rests",
                vec!["15.3"],
            ),
            EarMasterLesson::new("15.5", "3/4 - 1 bar: 8th & Quarter Triplets", vec!["15.4"]),
            EarMasterLesson::new(
                "15.6",
                "3/4 - 1 bar: 8th & Quarter Triplets - including rests",
                vec!["15.5"],
            ),
            EarMasterLesson::new(
                "15.7",
                "3/4 - 1 bar: Quarter notes, 8th & Quarter Triplets",
                vec!["15.6"],
            ),
            EarMasterLesson::new(
                "15.8",
                "3/4 - 1 bar: Quarter notes, 8th & Quarter Triplets - including rests",
                vec!["15.7"],
            ),
            EarMasterLesson::new(
                "15.9",
                "4/4 - 1 bar: Quarter notes, 16th Triplets",
                vec!["15.8"],
            ),
            EarMasterLesson::new(
                "15.10",
                "3/4 - 1 bar: Quarter notes, 16th Triplets - including rests",
                vec!["15.9"],
            ),
            EarMasterLesson::new(
                "15.11",
                "3/4 - 1 bar: 16th Triplets - including rests",
                vec!["15.10"],
            ),
            EarMasterLesson::new("15.12", "3/4 - 1 bar: 16th & 8th Triplets", vec!["15.11"]),
            EarMasterLesson::new(
                "15.13",
                "3/4 - 1 bar: 16th & 8th Triplets - including rests",
                vec!["15.12"],
            ),
            EarMasterLesson::new(
                "15.14",
                "3/4 - 1 bar: 16th, 8th, Quarter & Half Triplets",
                vec!["15.13"],
            ),
            EarMasterLesson::new(
                "15.15",
                "3/4 - 1 bar: 16th, 8th, Quarter & Half Triplets - including rests",
                vec!["15.14"],
            ),
            // Unit 16 - Introducing a new time signature: 5/4
            EarMasterLesson::new(
                "16.1",
                "5/4 - 1 bar: 8th & Quarter notes - including rests",
                vec!["14.17", "15.15"],
            ),
            EarMasterLesson::new(
                "16.2",
                "5/4 - 1 bar: 8th & Quarter notes - including rests",
                vec!["16.1"],
            ),
            EarMasterLesson::new(
                "16.3",
                "5/4 - 1 bar: 8th, Quarter & Half notes - including rests",
                vec!["16.2"],
            ),
            EarMasterLesson::new(
                "16.4",
                "5/4 - 1 bar: 16th, 8th & Quarter notes - including rests",
                vec!["16.3"],
            ),
            EarMasterLesson::new(
                "16.5",
                "5/4 - 1 bar: 16th, 8th & Quarter notes - including rests",
                vec!["16.4"],
            ),
            EarMasterLesson::new(
                "16.6",
                "5/4 - 1 bar: 16th, 8th, Quarter, Half & Whole notes - including rests",
                vec!["16.5"],
            ),
            EarMasterLesson::new(
                "16.7",
                "5/4 - 1 bar: 16th, 8th, Quarter, Half & Whole notes, 16th, 8th & Quarter Triplets",
                vec!["16.6"],
            ),
            EarMasterLesson::new(
                "16.8",
                "5/4 - 1 bar: 16th, 8th, Quarter, Half & Whole notes, 16th, 8th & Quarter Triplets - including rests",
                vec!["16.7"],
            ),
            // Unit 17 - Introducing new time signatures: 3/8, 4/8 & 6/8
            EarMasterLesson::new(
                "17.1",
                "3/8, 4/8 & 6/8 - 1 bar: 16th & 8th notes",
                vec!["16.8"],
            ),
            EarMasterLesson::new(
                "17.2",
                "3/8, 4/8 & 6/8 - 1 bar: 16th & 8th notes - including rests",
                vec!["17.1"],
            ),
            EarMasterLesson::new(
                "17.3",
                "3/8, 4/8 & 6/8 - 1 bar: 16th, 8th & Quarter notes",
                vec!["17.2"],
            ),
            EarMasterLesson::new(
                "17.4",
                "3/8, 4/8 & 6/8 - 1 bar: 16th, 8th & Quarter notes - including rests",
                vec!["17.3"],
            ),
            // Unit 18 - 6/8 with dotted subdivision
            EarMasterLesson::new(
                "18.1",
                "6/8 - 1 bar: 8th notes, Quarter dotted notes",
                vec!["17.4"],
            ),
            EarMasterLesson::new(
                "18.2",
                "6/8 - 1 bar: 8th notes, Quarter dotted notes - including rests",
                vec!["18.1"],
            ),
            EarMasterLesson::new(
                "18.3",
                "6/8 - 1 bar: 8th & Quarter notes, Quarter dotted notes",
                vec!["18.2"],
            ),
            EarMasterLesson::new(
                "18.4",
                "6/8 - 1 bar: 8th & Quarter notes, Quarter dotted notes - including rests",
                vec!["18.3"],
            ),
            EarMasterLesson::new(
                "18.5",
                "6/8 - 1 bar: 16th, 8th & Quarter notes, Quarter dotted notes - including rests",
                vec!["18.4"],
            ),
            // Introduction to the 9/8 time signature
            EarMasterLesson::new(
                "19.1",
                "9/8 - 1 bar: 16th & 8th notes",
                vec!["18.5"],
            ),
            EarMasterLesson::new(
                "19.2",
                "9/8 - 1 bar: 16th & 8th notes - including rests",
                vec!["19.1"],
            ),
            EarMasterLesson::new(
                "19.3",
                "9/8 - 1 bar: 16th, 8th & Quarter notes",
                vec!["19.2"],
            ),
            EarMasterLesson::new(
                "19.4",
                "9/8 - 1 bar: 16th, 8th & Quarter notes - including rests",
                vec!["19.3"],
            ),
            EarMasterLesson::new(
                "19.5",
                "9/8 - 1 bar: 8th notes, Quarter dotted notes",
                vec!["19.4"],
            ),
            EarMasterLesson::new(
                "19.6",
                "9/8 - 1 bar: 8th notes, Quarter dotted notes - including rests",
                vec!["19.5"],
            ),
            EarMasterLesson::new(
                "19.7",
                "9/8 - 1 bar: 8th & Quarter notes, Quarter dotted notes",
                vec!["19.6"],
            ),
            EarMasterLesson::new(
                "19.8",
                "9/8 - 1 bar: 8th & Quarter notes, Quarter dotted notes - including rests",
                vec!["19.7"],
            ),
            EarMasterLesson::new(
                "19.9",
                "9/8 - 1 bar: 16th, 8th & Quarter notes, Quarter dotted notes - including rests",
                vec!["19.8"],
            ),
            // Introduction to the 12/8 time signature
            EarMasterLesson::new(
                "20.1",
                "12/8 - 1 bar: 16th & 8th notes",
                vec!["19.9"],
            ),
            EarMasterLesson::new(
                "20.2",
                "12/8 - 1 bar: 16th & 8th notes - including rests",
                vec!["20.1"],
            ),
            EarMasterLesson::new(
                "20.3",
                "12/8 - 1 bar: 16th, 8th & Quarter notes",
                vec!["20.2"],
            ),
            EarMasterLesson::new(
                "20.4",
                "12/8 - 1 bar: 16th, 8th & Quarter notes - including rests",
                vec!["20.3"],
            ),
            EarMasterLesson::new(
                "20.5",
                "12/8 - 1 bar: 8th notes, Quarter dotted notes",
                vec!["20.4"],
            ),
            EarMasterLesson::new(
                "20.6",
                "12/8 - 1 bar: 8th notes, Quarter dotted notes - including rests",
                vec!["20.5"],
            ),
            EarMasterLesson::new(
                "20.7",
                "12/8 - 1 bar: 8th & Quarter notes, Quarter dotted notes",
                vec!["20.6"],
            ),
            EarMasterLesson::new(
                "20.8",
                "12/8 - 1 bar: 8th & Quarter notes, Quarter dotted notes - including rests",
                vec!["20.7"],
            ),
            EarMasterLesson::new(
                "20.9",
                "12/8 - 1 bar: 16th, 8th & Quarter notes, Quarter dotted notes - including rests",
                vec!["20.8"],
            ),
            // Unit 21 - Introducing new time signatures: 5/8 & 7/8
            EarMasterLesson::new(
                "21.1",
                "5/8 & 7/8 - 1 bar: 16th & 8th notes",
                vec!["20.9"],
            ),
            EarMasterLesson::new(
                "21.2",
                "5/8 & 7/8 - 1 bar: 16th & 8th notes - including rests",
                vec!["21.1"],
            ),
            EarMasterLesson::new(
                "21.3",
                "5/8 & 7/8 - 1 bar: 16th, 8th & Quarter notes",
                vec!["21.2"],
            ),
            EarMasterLesson::new(
                "21.4",
                "5/8 & 7/8 - 1 bar: 16th, 8th & Quarter notes - including rests",
                vec!["21.3"],
            ),
            // Unit 22 - Introducing new time signatures: 2/2, 3/2 & 4/2
            EarMasterLesson::new(
                "22.1",
                "2/2, 3/2 & 4/2 - 1 bar: Half & Whole notes",
                vec!["21.4"],
            ),
            EarMasterLesson::new(
                "22.2",
                "2/2, 3/2 & 4/2 - 1 bar: Half & Whole notes - including rests",
                vec!["22.1"],
            ),
            EarMasterLesson::new(
                "22.3",
                "2/2, 3/2 & 4/2 - 1 bar: Quarter, Half & Whole notes",
                vec!["22.2"],
            ),
            EarMasterLesson::new(
                "22.4",
                "2/2, 3/2 & 4/2 - 1 bar: Quarter, Half & Whole notes - including rests",
                vec!["22.3"],
            ),
            EarMasterLesson::new(
                "22.5",
                "2/2, 3/2 & 4/2 - 1 bar: 8th, Quarter & Half notes",
                vec!["22.4"],
            ),
            EarMasterLesson::new(
                "22.6",
                "2/2, 3/2 & 4/2 - 1 bar: 8th, Quarter & Half notes - including rests",
                vec!["22.5"],
            ),
            // Unit 23 - 4/4: Mixed note groupings
            EarMasterLesson::new(
                "23.1",
                "4/4 - 1 bar: 8th notes, 8th triplets",
                vec!["22.6"],
            ),
            EarMasterLesson::new(
                "23.2",
                "4/4 - 1 bar: 8th notes, 8th triplets - including rests",
                vec!["23.1"],
            ),
            EarMasterLesson::new(
                "23.3",
                "4/4 - 1 bar: Quarter notes, Quarter triplets",
                vec!["23.2"],
            ),
            EarMasterLesson::new(
                "23.4",
                "4/4 - 1 bar: Quarter notes, Quarter triplets - including rests",
                vec!["23.3"],
            ),
            EarMasterLesson::new(
                "23.5",
                "4/4 - 1 bar: 8th & Quarter notes, 8th & Quarter triplets",
                vec!["23.4"],
            ),
            EarMasterLesson::new(
                "23.6",
                "4/4 - 1 bar: 8th & Quarter notes, 8th & Quarter triplets - including rests",
                vec!["23.5"],
            ),
            EarMasterLesson::new(
                "23.7",
                "4/4 - 1 bar: 16th notes, 16th triplets",
                vec!["23.6"],
            ),
            EarMasterLesson::new(
                "23.8",
                "4/4 - 1 bar: 16th notes, 16th triplets - including rests",
                vec!["23.7"],
            ),
            EarMasterLesson::new(
                "23.9",
                "4/4 - 1 bar: 16th & 8th notes, 16th & 8th triplets",
                vec!["23.8"],
            ),
            EarMasterLesson::new(
                "23.10",
                "4/4 - 1 bar: 16th & 8th notes, 16th & 8th triplets - including rests",
                vec!["23.9"],
            ),
            EarMasterLesson::new(
                "23.11",
                "4/4 - 1 bar: 16th, 8th & Quarter notes, 16th, 8th & Quarter triplets",
                vec!["23.10"],
            ),
            EarMasterLesson::new(
                "23.12",
                "4/4 - 1 bar: 16th, 8th & Quarter notes, 16th, 8th & Quarter triplets - including rests",
                vec!["23.11"],
            ),
            // Unit 24 - 3/4: Mixed note groupings
            EarMasterLesson::new(
                "24.1",
                "3/4 - 1 bar: 8th notes, 8th triplets",
                vec!["22.6"],
            ),
            EarMasterLesson::new(
                "24.2",
                "3/4 - 1 bar: 8th notes, 8th triplets - including rests",
                vec!["24.1"],
            ),
            EarMasterLesson::new(
                "24.3",
                "3/4 - 1 bar: Quarter notes, Quarter triplets",
                vec!["24.2"],
            ),
            EarMasterLesson::new(
                "24.4",
                "3/4 - 1 bar: Quarter notes, Quarter triplets - including rests",
                vec!["24.3"],
            ),
            EarMasterLesson::new(
                "24.5",
                "3/4 - 1 bar: 8th & Quarter notes, 8th & Quarter triplets",
                vec!["24.4"],
            ),
            EarMasterLesson::new(
                "24.6",
                "3/4 - 1 bar: 8th & Quarter notes, 8th & Quarter triplets - including rests",
                vec!["24.5"],
            ),
            EarMasterLesson::new(
                "24.7",
                "3/4 - 1 bar: 16th notes, 16th triplets",
                vec!["24.6"],
            ),
            EarMasterLesson::new(
                "24.8",
                "3/4 - 1 bar: 16th notes, 16th triplets - including rests",
                vec!["24.7"],
            ),
            EarMasterLesson::new(
                "24.9",
                "3/4 - 1 bar: 16th & 8th notes, 16th & 8th triplets",
                vec!["24.8"],
            ),
            EarMasterLesson::new(
                "24.10",
                "3/4 - 1 bar: 16th & 8th notes, 16th & 8th triplets - including rests",
                vec!["24.9"],
            ),
            EarMasterLesson::new(
                "24.11",
                "3/4 - 1 bar: 16th, 8th & Quarter notes, 16th, 8th & Quarter triplets",
                vec!["24.10"],
            ),
            EarMasterLesson::new(
                "24.12",
                "3/4 - 1 bar: 16th, 8th & Quarter notes, 16th, 8th & Quarter triplets - including rests",
                vec!["24.11"],
            ),
            // Unit 25 - 4/4: Intrroducing 32nd notes
            EarMasterLesson::new(
                "25.1",
                "4/4 - 1 bar: 8th & Quarter notes - including rests",
                vec!["23.12"],
            ),
            EarMasterLesson::new(
                "25.2",
                "4/4 - 1 bar: 16th & 8th - including rests",
                vec!["25.1"],
            ),
            EarMasterLesson::new(
                "25.3",
                "4/4 - 1 bar: 32nd & 16th notes",
                vec!["25.2"],
            ),
            EarMasterLesson::new(
                "25.4",
                "4/4 - 1 bar: 32nd & 16th notes - including rests",
                vec!["25.3"],
            ),
            EarMasterLesson::new(
                "25.5",
                "4/4 - 1 bar: 32nd & 16th notes - including rests",
                vec!["25.4"],
            ),
            EarMasterLesson::new(
                "25.6",
                "4/4 - 1 bar: 32nd notes - including rests",
                vec!["25.5"],
            ),
            EarMasterLesson::new(
                "25.7",
                "4/4 - 1 bar: 32nd, 16th, 8th & Quarter notes",
                vec!["25.6"],
            ),
            EarMasterLesson::new(
                "25.8",
                "4/4 - 1 bar: 32nd, 16th, 8th & Quarter notes - including rests",
                vec!["25.7"],
            ),
            // Unit 26 - 3/4: 32nd notes
            EarMasterLesson::new(
                "26.1",
                "3/4 - 1 bar: 8th & Quarter notes - including rests",
                vec!["24.12"],
            ),
            EarMasterLesson::new(
                "26.2",
                "3/4 - 1 bar: 16th & 8th - including rests",
                vec!["26.1"],
            ),
            EarMasterLesson::new(
                "26.3",
                "3/4 - 1 bar: 32nd & 16th notes",
                vec!["26.2"],
            ),
            EarMasterLesson::new(
                "26.4",
                "3/4 - 1 bar: 32nd & 16th notes - including rests",
                vec!["26.3"],
            ),
            EarMasterLesson::new(
                "26.5",
                "3/4 - 1 bar: 32nd & 16th notes - including rests",
                vec!["26.4"],
            ),
            EarMasterLesson::new(
                "26.6",
                "3/4 - 1 bar: 32nd notes - including rests",
                vec!["26.5"],
            ),
            EarMasterLesson::new(
                "26.7",
                "3/4 - 1 bar: 32nd, 16th, 8th & Quarter notes",
                vec!["26.6"],
            ),
            EarMasterLesson::new(
                "26.8",
                "3/4 - 1 bar: 32nd, 16th, 8th & Quarter notes - including rests",
                vec!["26.7"],
            ),
            // Unit 27 - All time signatures from the previous lessons
            EarMasterLesson::new(
                "27.1",
                "Mixed time signatures with 32nd, 16th, 8th & Quarter notes",
                vec!["25.8", "26.8"],
            ),
            EarMasterLesson::new(
                "27.2",
                "Mixed time signatures with 32nd, 16th, 8th & Quarter notes - including rests",
                vec!["27.1"],
            ),
        ],
    };
    earmaster_course.course_builder()
}
