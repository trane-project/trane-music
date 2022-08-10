use std::collections::BTreeMap;

use lazy_static::lazy_static;
use trane::course_builder::CourseBuilder;
use ustr::Ustr;

use super::{EarMasterCourse, EarMasterLesson};

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::music::earmaster::rhythm_error_detection");
}

pub fn course_builder() -> CourseBuilder {
    let earmaster_course = EarMasterCourse {
        id: *COURSE_ID,
        name: "Rhythm Error Detection".to_string(),
        directory_name: "earmaster_error_detection".to_string(),
        metadata: Some(BTreeMap::from([])),
        lessons: vec![
            // Unit 1 - 4/4 - 2 bars: Whole, Half, Quarter & Eighth notes
            EarMasterLesson::new(
                "1.1",
                "4/4 - 2 bars: Quarter notes - including rests",
                vec![],
            ),
            EarMasterLesson::new("1.2", "4/4 - 2 bars: 8th & Quarter notes", vec!["1.1"]),
            EarMasterLesson::new(
                "1.3",
                "4/4 - 2 bars: 8th notes - including rests",
                vec!["1.2"],
            ),
            EarMasterLesson::new(
                "1.4",
                "4/4 - 2 bars: 8th & Quarter notes - including rests",
                vec!["1.3"],
            ),
            EarMasterLesson::new(
                "1.5",
                "4/4 - 2 bars: Quarter & Half Notes - including rests",
                vec!["1.4"],
            ),
            EarMasterLesson::new(
                "1.6",
                "4/4 - 2 bars: Half notes & Whole notes - including rests",
                vec!["1.5"],
            ),
            EarMasterLesson::new(
                "1.7",
                "4/4 - 2 bars: Quarter, Half & Whole notes - including rests",
                vec!["1.6"],
            ),
            EarMasterLesson::new(
                "1.8",
                "4/4 - 2 bars: 8th, Quarter, Half & Whole notes - including rests",
                vec!["1.7"],
            ),
            // Unit 2 - 3/4 - 2 bars: Half, Quarter & Eighth notes
            EarMasterLesson::new(
                "2.1",
                "3/4 - 2 bars: Quarter notes - including rests",
                vec![],
            ),
            EarMasterLesson::new("2.2", "3/4 - 2 bars: 8th & Quarter notes", vec!["2.1"]),
            EarMasterLesson::new(
                "2.3",
                "3/4 - 2 bars: 8th notes - including rests",
                vec!["2.2"],
            ),
            EarMasterLesson::new(
                "2.4",
                "3/4 - 2 bars: 8th & Quarter notes - including rests",
                vec!["2.3"],
            ),
            EarMasterLesson::new(
                "2.5",
                "3/4 - 2 bars: Quarter & Half Notes - including rests",
                vec!["2.4"],
            ),
            EarMasterLesson::new(
                "2.6",
                "3/4 - 2 bars: 8th, Quarter & Half notes - including rests",
                vec!["2.5"],
            ),
            // Unit 3 - 2/4 - 2 bars: Half, Quarter & Eighth notes
            EarMasterLesson::new(
                "3.1",
                "2/4 - 2 bars: Quarter notes - including rests",
                vec![],
            ),
            EarMasterLesson::new("3.2", "2/4 - 2 bars: 8th & Quarter notes", vec!["3.1"]),
            EarMasterLesson::new(
                "3.3",
                "2/4 - 2 bars: 8th notes - including rests",
                vec!["3.2"],
            ),
            EarMasterLesson::new(
                "3.4",
                "2/4 - 2 bars: 8th & Quarter notes - including rests",
                vec!["3.3"],
            ),
            EarMasterLesson::new(
                "3.5",
                "2/4 - 2 bars: Quarter & Half Notes - including rests",
                vec!["3.4"],
            ),
            EarMasterLesson::new(
                "3.6",
                "2/4 - 2 bars: 8th, Quarter & Half notes - including rests",
                vec!["3.5"],
            ),
            // Unit 4 - 4/4 - 4 bars: Whole, Half, Quarter & Eighth notes
            EarMasterLesson::new(
                "4.1",
                "4/4 - 4 bars: Quarter notes - including rests",
                vec!["1.8"],
            ),
            EarMasterLesson::new("4.2", "4/4 - 4 bars: 8th & Quarter notes", vec!["4.1"]),
            EarMasterLesson::new(
                "4.3",
                "4/4 - 4 bars: 8th notes - including rests",
                vec!["4.2"],
            ),
            EarMasterLesson::new(
                "4.4",
                "4/4 - 4 bars: 8th & Quarter notes - including rests",
                vec!["4.3"],
            ),
            EarMasterLesson::new(
                "4.5",
                "4/4 - 4 bars: Quarter & Half Notes - including rests",
                vec!["4.4"],
            ),
            EarMasterLesson::new(
                "4.6",
                "4/4 - 4 bars: Half notes & Whole notes - including rests",
                vec!["4.5"],
            ),
            EarMasterLesson::new(
                "4.7",
                "4/4 - 4 bars: Quarter, Half & Whole notes - including rests",
                vec!["4.6"],
            ),
            EarMasterLesson::new(
                "4.8",
                "4/4 - 4 bars: 8th, Quarter, Half & Whole notes - including rests",
                vec!["4.7"],
            ),
            // Unit 5 - 3/4 - 4 bars: Half, Quarter & Eighth notes
            EarMasterLesson::new(
                "5.1",
                "3/4 - 4 bars: Quarter notes - including rests",
                vec!["2.6"],
            ),
            EarMasterLesson::new("5.2", "3/4 - 4 bars: 8th & Quarter notes", vec!["5.1"]),
            EarMasterLesson::new(
                "5.3",
                "3/4 - 4 bars: 8th notes - including rests",
                vec!["5.2"],
            ),
            EarMasterLesson::new(
                "5.4",
                "3/4 - 4 bars: 8th & Quarter notes - including rests",
                vec!["5.3"],
            ),
            EarMasterLesson::new(
                "5.5",
                "3/4 - 4 bars: Quarter & Half Notes - including rests",
                vec!["5.4"],
            ),
            EarMasterLesson::new(
                "5.6",
                "3/4 - 4 bars: 8th, Quarter & Half notes - including rests",
                vec!["5.5"],
            ),
            // Unit 6 - 2/4 - 4 bars: Half, Quarter & Eighth notes
            EarMasterLesson::new(
                "6.1",
                "2/4 - 4 bars: Quarter notes - including rests",
                vec!["3.6"],
            ),
            EarMasterLesson::new("6.2", "2/4 - 4 bars: 8th & Quarter notes", vec!["6.1"]),
            EarMasterLesson::new(
                "6.3",
                "2/4 - 4 bars: 8th notes - including rests",
                vec!["6.2"],
            ),
            EarMasterLesson::new(
                "6.4",
                "2/4 - 4 bars: 8th & Quarter notes - including rests",
                vec!["6.3"],
            ),
            EarMasterLesson::new(
                "6.5",
                "2/4 - 4 bars: Quarter & Half Notes - including rests",
                vec!["6.4"],
            ),
            EarMasterLesson::new(
                "6.6",
                "2/4 - 4 bars: 8th, Quarter & Half notes - including rests",
                vec!["6.5"],
            ),
            // Unit 7 - 8 bars: Whole, Half, Quarter & Eighth notes
            EarMasterLesson::new(
                "7.1",
                "8 bars: Quarter notes - including rests",
                vec!["4.8"],
            ),
            EarMasterLesson::new("7.2", "8 bars: 8th & Quarter notes", vec!["7.1"]),
            EarMasterLesson::new("7.3", "8 bars: 8th notes - including rests", vec!["7.2"]),
            EarMasterLesson::new(
                "7.4",
                "8 bars: 8th & Quarter notes - including rests",
                vec!["7.3"],
            ),
            EarMasterLesson::new(
                "7.5",
                "8 bars: Quarter & Half Notes - including rests",
                vec!["7.4"],
            ),
            EarMasterLesson::new(
                "7.6",
                "8 bars: Half & Whole notes - including rests",
                vec!["7.5"],
            ),
            EarMasterLesson::new(
                "7.7",
                "8 bars: Quarter, Half & Whole notes - including rests",
                vec!["7.6"],
            ),
            EarMasterLesson::new(
                "7.8",
                "8 bars: 8th, Quarter, Half & Whole notes - including rests",
                vec!["7.7"],
            ),
            // Unit 8 - 4/4: Introducing Sixteenth Note Groupings
            EarMasterLesson::new(
                "8.1",
                "4/4 - 2 bars: 16th & Quarter notes - including rests",
                vec!["4.8"],
            ),
            EarMasterLesson::new(
                "8.2",
                "4/4 - 2 bars: 16th, 8th & Quarter notes - including rests",
                vec!["8.1"],
            ),
            EarMasterLesson::new(
                "8.3",
                "4/4 - 2 bars: 16th, 8th & Quarter notes - including rests",
                vec!["8.2"],
            ),
            // Unit 9 - 3/4: Introducing Sixteenth Note Groupings
            EarMasterLesson::new(
                "9.1",
                "3/4 - 2 bars: 16th & Quarter notes - including rests",
                vec!["5.6"],
            ),
            EarMasterLesson::new(
                "9.2",
                "3/4 - 2 bars: 16th, 8th & Quarter notes - including rests",
                vec!["9.1"],
            ),
            EarMasterLesson::new(
                "9.3",
                "3/4 - 2 bars: 16th, 8th & Quarter notes - including rests",
                vec!["9.2"],
            ),
            // Unit 10 - 2/4: Introducing Sixteenth Note Groupings
            EarMasterLesson::new(
                "10.1",
                "2/4 - 2 bars: 16th & Quarter notes - including rests",
                vec!["6.6"],
            ),
            EarMasterLesson::new(
                "10.2",
                "2/4 - 2 bars: 16th, 8th & Quarter notes - including rests",
                vec!["10.1"],
            ),
            EarMasterLesson::new(
                "10.3",
                "2/4 - 2 bars: 16th, 8th & Quarter notes - including rests",
                vec!["10.2"],
            ),
            // Unit 11 - Advanced Sixteenth Note Groupings
            EarMasterLesson::new(
                "11.1",
                "4/4 - 2 bars: 16th, 8th & Quarter notes",
                vec!["8.3"],
            ),
            EarMasterLesson::new(
                "11.2",
                "3/4 - 2 bars: 16th, 8th & Quarter notes",
                vec!["9.3"],
            ),
            EarMasterLesson::new(
                "11.3",
                "2/4 - 2 bars: 16th, 8th & Quarter notes",
                vec!["10.3"],
            ),
            EarMasterLesson::new(
                "11.4",
                "2/4, 3/4 & 4/4 - 2 bars: 16th, 8th & Quarter notes",
                vec!["11.1", "11.2", "11.3"],
            ),
            EarMasterLesson::new(
                "11.5",
                "4/4 - 2 bars: 16th, 8th & Quarter notes",
                vec!["11.4"],
            ),
            EarMasterLesson::new(
                "11.6",
                "3/4 - 2 bars: 16th, 8th & Quarter notes",
                vec!["11.4"],
            ),
            EarMasterLesson::new(
                "11.7",
                "2/4 - 2 bars: 16th, 8th & Quarter notes",
                vec!["11.4"],
            ),
            EarMasterLesson::new(
                "11.8",
                "2/4, 3/4 & 4/4 - 2 bars: 16th, 8th & Quarter notes",
                vec!["11.5", "11.6", "11.7"],
            ),
            // Unit 12 - Advanced Sixteenth Note Groupings with Rests
            EarMasterLesson::new(
                "12.1",
                "4/4 - 2 bars: 16th, 8th & Quarter notes - including rests",
                vec!["11.8"],
            ),
            EarMasterLesson::new(
                "12.2",
                "3/4 - 2 bars: 16th, 8th & Quarter notes - including rests",
                vec!["11.8"],
            ),
            EarMasterLesson::new(
                "12.3",
                "2/4 - 2 bars: 16th, 8th & Quarter notes - including rests",
                vec!["11.8"],
            ),
            EarMasterLesson::new(
                "12.4",
                "2/4, 3/4 & 4/4 - 2 bars: 16th, 8th & Quarter notes - including rests",
                vec!["12.1", "12.2", "12.3"],
            ),
            EarMasterLesson::new(
                "12.5",
                "4/4 - 2 bars: 16th, 8th & Quarter notes - including rests",
                vec!["12.4"],
            ),
            EarMasterLesson::new(
                "12.6",
                "3/4 - 2 bars: 16th, 8th & Quarter notes - including rests",
                vec!["12.4"],
            ),
            EarMasterLesson::new(
                "12.7",
                "2/4 - 2 bars: 16th, 8th & Quarter notes - including rests",
                vec!["12.4"],
            ),
            EarMasterLesson::new(
                "12.8",
                "2/4, 3/4 & 4/4 - 2 bars: 16th, 8th & Quarter notes - including rests",
                vec!["12.5", "12.6", "12.7"],
            ),
            // Unit 13 - Advanced 16th and 8th Combinations with Rests
            EarMasterLesson::new(
                "13.1",
                "4/4 - 2 bars: 16th, 8th & Quarter notes - including rests",
                vec!["12.8"],
            ),
            EarMasterLesson::new(
                "13.2",
                "3/4 - 2 bars: 16th, 8th & Quarter notes - including rests",
                vec!["12.8"],
            ),
            EarMasterLesson::new(
                "13.3",
                "2/4 - 2 bars: 16th, 8th & Quarter notes - including rests",
                vec!["12.8"],
            ),
            EarMasterLesson::new(
                "13.4",
                "2/4, 3/4 & 4/4 - 2 bars: 16th, 8th & Quarter notes - including rests",
                vec!["13.1", "13.2", "13.3"],
            ),
            EarMasterLesson::new(
                "13.5",
                "4/4 - 2 bars: 16th, 8th & Quarter notes - including rests",
                vec!["13.4"],
            ),
            EarMasterLesson::new(
                "13.6",
                "3/4 - 2 bars: 16th, 8th & Quarter notes - including rests",
                vec!["13.4"],
            ),
            EarMasterLesson::new(
                "13.7",
                "2/4 - 2 bars: 16th, 8th & Quarter notes - including rests",
                vec!["13.4"],
            ),
            EarMasterLesson::new(
                "13.8",
                "2/4, 3/4 & 4/4 - 2 bars: 16th, 8th & Quarter notes - including rests",
                vec!["13.5", "13.6", "13.7"],
            ),
            // Unit 14 - All note values from the previous lessons, including rests
            EarMasterLesson::new(
                "14.1",
                "4/4 - 2 bars: 16th, 8th, Quarter, Half & Whole notes - including rests",
                vec!["13.8"],
            ),
            EarMasterLesson::new(
                "14.2",
                "3/4 - 2 bars: 16th, 8th, Quarter & Half notes - including rests",
                vec!["13.8"],
            ),
            EarMasterLesson::new(
                "14.3",
                "2/4 - 2 bars: 16th, 8th, Quarter & Half notes - including rests",
                vec!["13.8"],
            ),
            EarMasterLesson::new(
                "14.4",
                "2/4, 3/4 & 4/4 - 2 bars: 16th, 8th, Quarter & Half notes - including rests",
                vec!["14.1", "14.2", "14.3"],
            ),
            // Unit 15 - 4/4: Triplets
            EarMasterLesson::new(
                "15.1",
                "4/4 - 2 bars: Quarter notes, 8th Triplets",
                vec!["14.4"],
            ),
            EarMasterLesson::new(
                "15.2",
                "4/4 - 2 bars: Quarter notes, 8th Triplets - including rests",
                vec!["15.1"],
            ),
            EarMasterLesson::new(
                "15.3",
                "4/4 - 2 bars: Quarter & Half notes, 8th Triplets - including rests",
                vec!["15.2"],
            ),
            EarMasterLesson::new(
                "15.4",
                "4/4 - 2 bars: Quarter notes, 8th Triplets - including rests",
                vec!["15.3"],
            ),
            EarMasterLesson::new("15.5", "4/4 - 2 bars: 8th & Quarter Triplets", vec!["15.4"]),
            EarMasterLesson::new(
                "15.6",
                "4/4 - 2 bars: 8th & Quarter Triplets - including rests",
                vec!["15.5"],
            ),
            EarMasterLesson::new(
                "15.7",
                "4/4 - 2 bars: Quarter notes, 8th & Quarter Triplets",
                vec!["15.6"],
            ),
            EarMasterLesson::new(
                "15.8",
                "4/4 - 2 bars: Quarter notes, 8th & Quarter Triplets - including rests",
                vec!["15.7"],
            ),
            EarMasterLesson::new("15.9", "4/4 - 2 bars: Half triplets", vec!["15.8"]),
            EarMasterLesson::new(
                "15.10",
                "4/4 - 2 bars: Half triplets - including rests",
                vec!["15.9"],
            ),
            EarMasterLesson::new(
                "15.11",
                "4/4 - 2 bars: Quarter notes, 16th Triplets",
                vec!["15.10"],
            ),
            EarMasterLesson::new(
                "15.12",
                "4/4 - 2 bars: Quarter notes, 16th Triplets - including rests",
                vec!["15.11"],
            ),
            EarMasterLesson::new(
                "15.13",
                "4/4 - 2 bars: 16th Triplets - including rests",
                vec!["15.12"],
            ),
            EarMasterLesson::new("15.14", "4/4 - 2 bars: 16th & 8th Triplets", vec!["15.13"]),
            EarMasterLesson::new(
                "15.15",
                "4/4 - 2 bars: 16th & 8th Triplets - including rests",
                vec!["15.14"],
            ),
            EarMasterLesson::new(
                "15.16",
                "4/4 - 2 bars: 16th, 8th, Quarter & Half Triplets",
                vec!["15.15"],
            ),
            EarMasterLesson::new(
                "15.17",
                "4/4 - 2 bars: 16th, 8th, Quarter & Half Triplets - including rests",
                vec!["15.16"],
            ),
            // Unit 16 - Triplets, now in 3/4
            EarMasterLesson::new(
                "16.1",
                "3/4 - 2 bars: Quarter notes, 8th Triplets",
                vec!["14.4"],
            ),
            EarMasterLesson::new(
                "16.2",
                "3/4 - 2 bars: Quarter notes, 8th Triplets - including rests",
                vec!["16.1"],
            ),
            EarMasterLesson::new(
                "16.3",
                "3/4 - 2 bars: Quarter & Half notes, 8th Triplets - including rests",
                vec!["16.2"],
            ),
            EarMasterLesson::new(
                "16.4",
                "3/4 - 2 bars: Quarter notes, 8th Triplets - including rests",
                vec!["16.3"],
            ),
            EarMasterLesson::new("16.5", "3/4 - 2 bars: 8th & Quarter Triplets", vec!["16.4"]),
            EarMasterLesson::new(
                "16.6",
                "3/4 - 2 bars: 8th & Quarter Triplets - including rests",
                vec!["16.5"],
            ),
            EarMasterLesson::new(
                "16.7",
                "3/4 - 2 bars: Quarter notes, 8th & Quarter Triplets",
                vec!["16.6"],
            ),
            EarMasterLesson::new(
                "16.8",
                "3/4 - 2 bars: Quarter notes, 8th & Quarter Triplets - including rests",
                vec!["16.7"],
            ),
            EarMasterLesson::new(
                "16.9",
                "4/4 - 2 bars: Quarter notes, 16th Triplets",
                vec!["16.8"],
            ),
            EarMasterLesson::new(
                "16.10",
                "3/4 - 2 bars: Quarter notes, 16th Triplets - including rests",
                vec!["16.9"],
            ),
            EarMasterLesson::new(
                "16.11",
                "3/4 - 2 bars: 16th Triplets - including rests",
                vec!["16.10"],
            ),
            EarMasterLesson::new("16.12", "3/4 - 2 bars: 16th & 8th Triplets", vec!["16.11"]),
            EarMasterLesson::new(
                "16.13",
                "3/4 - 2 bars: 16th & 8th Triplets - including rests",
                vec!["16.12"],
            ),
            EarMasterLesson::new(
                "16.14",
                "3/4 - 2 bars: 16th, 8th, Quarter & Half Triplets",
                vec!["16.13"],
            ),
            EarMasterLesson::new(
                "16.15",
                "3/4 - 2 bars: 16th, 8th, Quarter & Half Triplets - including rests",
                vec!["16.14"],
            ),
            // Unit 16 - Introducing a new time signature: 5/4
            EarMasterLesson::new(
                "17.1",
                "5/4 - 2 bars: 8th & Quarter notes - including rests",
                vec!["15.17", "16.15"],
            ),
            EarMasterLesson::new(
                "17.2",
                "5/4 - 2 bars: 8th & Quarter notes - including rests",
                vec!["17.1"],
            ),
            EarMasterLesson::new(
                "17.3",
                "5/4 - 2 bars: 8th, Quarter & Half notes - including rests",
                vec!["17.2"],
            ),
            EarMasterLesson::new(
                "17.4",
                "5/4 - 2 bars: 16th, 8th & Quarter notes - including rests",
                vec!["17.3"],
            ),
            EarMasterLesson::new(
                "17.5",
                "5/4 - 2 bars: 16th, 8th & Quarter notes - including rests",
                vec!["17.4"],
            ),
            EarMasterLesson::new(
                "17.6",
                "5/4 - 2 bars: 16th, 8th, Quarter, Half & Whole notes - including rests",
                vec!["17.5"],
            ),
            EarMasterLesson::new(
                "17.7",
                "5/4 - 2 bars: 16th, 8th, Quarter, Half & Whole notes, 16th, 8th & Quarter Triplets",
                vec!["17.6"],
            ),
            EarMasterLesson::new(
                "17.8",
                "5/4 - 2 bars: 16th, 8th, Quarter, Half & Whole notes, 16th, 8th & Quarter Triplets - including rests",
                vec!["17.7"],
            ),
            // Unit 18 - Introducing new time signatures: 3/8, 4/8 & 6/8
            EarMasterLesson::new(
                "18.1",
                "3/8, 4/8 & 6/8 - 2 bars: 16th & 8th notes",
                vec!["17.8"],
            ),
            EarMasterLesson::new(
                "18.2",
                "3/8, 4/8 & 6/8 - 2 bars: 16th & 8th notes - including rests",
                vec!["18.1"],
            ),
            EarMasterLesson::new(
                "18.3",
                "3/8, 4/8 & 6/8 - 2 bars: 16th, 8th & Quarter notes",
                vec!["18.2"],
            ),
            EarMasterLesson::new(
                "18.4",
                "3/8, 4/8 & 6/8 - 2 bars: 16th, 8th & Quarter notes - including rests",
                vec!["18.3"],
            ),
            // Unit 19 - 6/8 with dotted subdivision
            EarMasterLesson::new(
                "19.1",
                "6/8 - 2 bars: 8th notes, Quarter dotted notes",
                vec!["18.4"],
            ),
            EarMasterLesson::new(
                "19.2",
                "6/8 - 2 bars: 8th notes, Quarter dotted notes - including rests",
                vec!["19.1"],
            ),
            EarMasterLesson::new(
                "19.3",
                "6/8 - 2 bars: 8th & Quarter notes, Quarter dotted notes",
                vec!["19.2"],
            ),
            EarMasterLesson::new(
                "19.4",
                "6/8 - 2 bars: 8th & Quarter notes, Quarter dotted notes - including rests",
                vec!["19.3"],
            ),
            EarMasterLesson::new(
                "19.5",
                "6/8 - 2 bars: 16th, 8th & Quarter notes, Quarter dotted notes - including rests",
                vec!["19.4"],
            ),
            // Unit 20 - Introduction to the 9/8 time signature
            EarMasterLesson::new(
                "20.1",
                "9/8 - 2 bars: 16th & 8th notes",
                vec!["19.5"],
            ),
            EarMasterLesson::new(
                "20.2",
                "9/8 - 2 bars: 16th & 8th notes - including rests",
                vec!["20.1"],
            ),
            EarMasterLesson::new(
                "20.3",
                "9/8 - 2 bars: 16th, 8th & Quarter notes",
                vec!["20.2"],
            ),
            EarMasterLesson::new(
                "20.4",
                "9/8 - 2 bars: 16th, 8th & Quarter notes - including rests",
                vec!["20.3"],
            ),
            EarMasterLesson::new(
                "20.5",
                "9/8 - 2 bars: 8th notes, Quarter dotted notes",
                vec!["20.4"],
            ),
            EarMasterLesson::new(
                "20.6",
                "9/8 - 2 bars: 8th notes, Quarter dotted notes - including rests",
                vec!["20.5"],
            ),
            EarMasterLesson::new(
                "20.7",
                "9/8 - 2 bars: 8th & Quarter notes, Quarter dotted notes",
                vec!["20.6"],
            ),
            EarMasterLesson::new(
                "20.8",
                "9/8 - 2 bars: 8th & Quarter notes, Quarter dotted notes - including rests",
                vec!["20.7"],
            ),
            EarMasterLesson::new(
                "20.9",
                "9/8 - 2 bars: 16th, 8th & Quarter notes, Quarter dotted notes - including rests",
                vec!["20.8"],
            ),
            // Unit 21 - Introduction to the 12/8 time signature
            EarMasterLesson::new(
                "21.1",
                "12/8 - 2 bars: 16th & 8th notes",
                vec!["20.9"],
            ),
            EarMasterLesson::new(
                "21.2",
                "12/8 - 2 bars: 16th & 8th notes - including rests",
                vec!["21.1"],
            ),
            EarMasterLesson::new(
                "21.3",
                "12/8 - 2 bars: 16th, 8th & Quarter notes",
                vec!["21.2"],
            ),
            EarMasterLesson::new(
                "21.4",
                "12/8 - 2 bars: 16th, 8th & Quarter notes - including rests",
                vec!["21.3"],
            ),
            EarMasterLesson::new(
                "21.5",
                "12/8 - 2 bars: 8th notes, Quarter dotted notes",
                vec!["21.4"],
            ),
            EarMasterLesson::new(
                "21.6",
                "12/8 - 2 bars: 8th notes, Quarter dotted notes - including rests",
                vec!["21.5"],
            ),
            EarMasterLesson::new(
                "21.7",
                "12/8 - 2 bars: 8th & Quarter notes, Quarter dotted notes",
                vec!["21.6"],
            ),
            EarMasterLesson::new(
                "21.8",
                "12/8 - 2 bars: 8th & Quarter notes, Quarter dotted notes - including rests",
                vec!["21.7"],
            ),
            EarMasterLesson::new(
                "21.9",
                "12/8 - 2 bars: 16th, 8th & Quarter notes, Quarter dotted notes - including rests",
                vec!["21.8"],
            ),
            // Unit 22 - Introducing new time signatures: 5/8 & 7/8
            EarMasterLesson::new(
                "22.1",
                "5/8 & 7/8 - 2 bars: 16th & 8th notes",
                vec!["21.9"],
            ),
            EarMasterLesson::new(
                "22.2",
                "5/8 & 7/8 - 2 bars: 16th & 8th notes - including rests",
                vec!["22.1"],
            ),
            EarMasterLesson::new(
                "22.3",
                "5/8 & 7/8 - 2 bars: 16th, 8th & Quarter notes",
                vec!["22.2"],
            ),
            EarMasterLesson::new(
                "22.4",
                "5/8 & 7/8 - 2 bars: 16th, 8th & Quarter notes - including rests",
                vec!["22.3"],
            ),
            // Unit 23 - Introducing new time signatures: 2/2, 3/2 & 4/2
            EarMasterLesson::new(
                "23.1",
                "2/2, 3/2 & 4/2 - 2 bars: Half & Whole notes",
                vec!["22.4"],
            ),
            EarMasterLesson::new(
                "23.2",
                "2/2, 3/2 & 4/2 - 2 bars: Half & Whole notes - including rests",
                vec!["23.1"],
            ),
            EarMasterLesson::new(
                "23.3",
                "2/2, 3/2 & 4/2 - 2 bars: Quarter, Half & Whole notes",
                vec!["23.2"],
            ),
            EarMasterLesson::new(
                "23.4",
                "2/2, 3/2 & 4/2 - 2 bars: Quarter, Half & Whole notes - including rests",
                vec!["23.3"],
            ),
            EarMasterLesson::new(
                "23.5",
                "2/2, 3/2 & 4/2 - 2 bars: 8th, Quarter & Half notes",
                vec!["23.4"],
            ),
            EarMasterLesson::new(
                "23.6",
                "2/2, 3/2 & 4/2 - 2 bars: 8th, Quarter & Half notes - including rests",
                vec!["23.5"],
            ),
            // Unit 24 - 4/4: Mixed note groupings
            EarMasterLesson::new(
                "24.1",
                "4/4 - 2 bars: 8th notes, 8th triplets",
                vec!["23.6"],
            ),
            EarMasterLesson::new(
                "24.2",
                "4/4 - 2 bars: 8th notes, 8th triplets - including rests",
                vec!["24.1"],
            ),
            EarMasterLesson::new(
                "24.3",
                "4/4 - 2 bars: Quarter notes, Quarter triplets",
                vec!["24.2"],
            ),
            EarMasterLesson::new(
                "24.4",
                "4/4 - 2 bars: Quarter notes, Quarter triplets - including rests",
                vec!["24.3"],
            ),
            EarMasterLesson::new(
                "24.5",
                "4/4 - 2 bars: 8th & Quarter notes, 8th & Quarter triplets",
                vec!["24.4"],
            ),
            EarMasterLesson::new(
                "24.6",
                "4/4 - 2 bars: 8th & Quarter notes, 8th & Quarter triplets - including rests",
                vec!["24.5"],
            ),
            EarMasterLesson::new(
                "24.7",
                "4/4 - 2 bars: 16th notes, 16th triplets",
                vec!["24.6"],
            ),
            EarMasterLesson::new(
                "24.8",
                "4/4 - 2 bars: 16th notes, 16th triplets - including rests",
                vec!["24.7"],
            ),
            EarMasterLesson::new(
                "24.9",
                "4/4 - 2 bars: 16th & 8th notes, 16th & 8th triplets",
                vec!["24.8"],
            ),
            EarMasterLesson::new(
                "24.10",
                "4/4 - 2 bars: 16th & 8th notes, 16th & 8th triplets - including rests",
                vec!["24.9"],
            ),
            EarMasterLesson::new(
                "24.11",
                "4/4 - 2 bars: 16th, 8th & Quarter notes, 16th, 8th & Quarter triplets",
                vec!["24.10"],
            ),
            EarMasterLesson::new(
                "24.12",
                "4/4 - 2 bars: 16th, 8th & Quarter notes, 16th, 8th & Quarter triplets - including rests",
                vec!["24.11"],
            ),
            // Unit 25 - 3/4: Mixed note groupings
            EarMasterLesson::new(
                "25.1",
                "3/4 - 2 bars: 8th notes, 8th triplets",
                vec!["23.6"],
            ),
            EarMasterLesson::new(
                "25.2",
                "3/4 - 2 bars: 8th notes, 8th triplets - including rests",
                vec!["25.1"],
            ),
            EarMasterLesson::new(
                "25.3",
                "3/4 - 2 bars: Quarter notes, Quarter triplets",
                vec!["25.2"],
            ),
            EarMasterLesson::new(
                "25.4",
                "3/4 - 2 bars: Quarter notes, Quarter triplets - including rests",
                vec!["25.3"],
            ),
            EarMasterLesson::new(
                "25.5",
                "3/4 - 2 bars: 8th & Quarter notes, 8th & Quarter triplets",
                vec!["25.4"],
            ),
            EarMasterLesson::new(
                "25.6",
                "3/4 - 2 bars: 8th & Quarter notes, 8th & Quarter triplets - including rests",
                vec!["25.5"],
            ),
            EarMasterLesson::new(
                "25.7",
                "3/4 - 2 bars: 16th notes, 16th triplets",
                vec!["25.6"],
            ),
            EarMasterLesson::new(
                "25.8",
                "3/4 - 2 bars: 16th notes, 16th triplets - including rests",
                vec!["25.7"],
            ),
            EarMasterLesson::new(
                "25.9",
                "3/4 - 2 bars: 16th & 8th notes, 16th & 8th triplets",
                vec!["25.8"],
            ),
            EarMasterLesson::new(
                "25.10",
                "3/4 - 2 bars: 16th & 8th notes, 16th & 8th triplets - including rests",
                vec!["25.9"],
            ),
            EarMasterLesson::new(
                "25.11",
                "3/4 - 2 bars: 16th, 8th & Quarter notes, 16th, 8th & Quarter triplets",
                vec!["25.10"],
            ),
            EarMasterLesson::new(
                "25.12",
                "3/4 - 2 bars: 16th, 8th & Quarter notes, 16th, 8th & Quarter triplets - including rests",
                vec!["25.11"],
            ),
            // Unit 26 - 4/4: Intrroducing 32nd notes
            EarMasterLesson::new(
                "26.1",
                "4/4 - 2 bars: 8th & Quarter notes - including rests",
                vec!["24.12"],
            ),
            EarMasterLesson::new(
                "26.2",
                "4/4 - 2 bars: 16th & 8th - including rests",
                vec!["26.1"],
            ),
            EarMasterLesson::new(
                "26.3",
                "4/4 - 2 bars: 32nd & 16th notes",
                vec!["26.2"],
            ),
            EarMasterLesson::new(
                "26.4",
                "4/4 - 2 bars: 32nd & 16th notes - including rests",
                vec!["26.3"],
            ),
            EarMasterLesson::new(
                "26.5",
                "4/4 - 2 bars: 32nd & 16th notes - including rests",
                vec!["26.4"],
            ),
            EarMasterLesson::new(
                "26.6",
                "4/4 - 2 bars: 32nd notes - including rests",
                vec!["26.5"],
            ),
            EarMasterLesson::new(
                "26.7",
                "4/4 - 2 bars: 32nd, 16th, 8th & Quarter notes",
                vec!["26.6"],
            ),
            EarMasterLesson::new(
                "26.8",
                "4/4 - 2 bars: 32nd, 16th, 8th & Quarter notes - including rests",
                vec!["26.7"],
            ),
            // Unit 27 - 3/4: 32nd notes
            EarMasterLesson::new(
                "27.1",
                "3/4 - 2 bars: 8th & Quarter notes - including rests",
                vec!["25.12"],
            ),
            EarMasterLesson::new(
                "27.2",
                "3/4 - 2 bars: 16th & 8th - including rests",
                vec!["27.1"],
            ),
            EarMasterLesson::new(
                "27.3",
                "3/4 - 2 bars: 32nd & 16th notes",
                vec!["27.2"],
            ),
            EarMasterLesson::new(
                "27.4",
                "3/4 - 2 bars: 32nd & 16th notes - including rests",
                vec!["27.3"],
            ),
            EarMasterLesson::new(
                "27.5",
                "3/4 - 2 bars: 32nd & 16th notes - including rests",
                vec!["27.4"],
            ),
            EarMasterLesson::new(
                "27.6",
                "3/4 - 2 bars: 32nd notes - including rests",
                vec!["27.5"],
            ),
            EarMasterLesson::new(
                "27.7",
                "3/4 - 2 bars: 32nd, 16th, 8th & Quarter notes",
                vec!["27.6"],
            ),
            EarMasterLesson::new(
                "27.8",
                "3/4 - 2 bars: 32nd, 16th, 8th & Quarter notes - including rests",
                vec!["27.7"],
            ),
            // Unit 28 - All time signatures from the previous lessons
            EarMasterLesson::new(
                "28.1",
                "Mixed time signatures with 32nd, 16th, 8th & Quarter notes",
                vec!["26.8", "27.8"],
            ),
            EarMasterLesson::new(
                "28.2",
                "Mixed time signatures with 32nd, 16th, 8th & Quarter notes - including rests",
                vec!["28.1"],
            ),
        ],
    };
    earmaster_course.course_builder()
}
