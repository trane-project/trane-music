use std::collections::BTreeMap;

use lazy_static::lazy_static;
use trane::course_builder::CourseBuilder;
use ustr::Ustr;

use super::{EarMasterCourse, EarMasterLesson};

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::music::earmaster::interval_singing");
}

pub fn course_builder() -> CourseBuilder {
    let earmaster_course = EarMasterCourse {
        id: *COURSE_ID,
        name: "Interval Singing".to_string(),
        directory_name: "earmaster_interval_singing".to_string(),
        metadata: Some(BTreeMap::from([])),
        lessons: vec![
            // Unit 1 - Ascending intervals from Do
            EarMasterLesson::new("1.1", "Do - Re", vec![]),
            EarMasterLesson::new("1.2", "Do - Mi", vec![]),
            EarMasterLesson::new("1.3", "Do - Re and Do - Mi", vec!["1.1", "1.2"]),
            EarMasterLesson::new("1.4", "Do - Fa", vec![]),
            EarMasterLesson::new("1.5", "Do - Sol", vec![]),
            EarMasterLesson::new("1.6", "Do - Fa and Do - So", vec!["1.4", "1.5"]),
            EarMasterLesson::new("1.7", "Do to Re, Mi, Fa, and So", vec!["1.3", "1.6"]),
            EarMasterLesson::new("1.8", "Do - La", vec!["1.7"]),
            EarMasterLesson::new("1.9", "Do - Ti", vec!["1.7"]),
            EarMasterLesson::new("1.10", "Do - Do", vec!["1.7"]),
            EarMasterLesson::new("1.11", "Do to La, Ti and Do", vec!["1.8", "1.9", "1.10"]),
            EarMasterLesson::new("1.12", "All ascending intervals from Do", vec!["1.11"]),
            // Unit 2 - Descending intervals from Do
            EarMasterLesson::new("2.1", "Do - Ti", vec!["1.12"]),
            EarMasterLesson::new("2.2", "Do - La", vec!["1.12"]),
            EarMasterLesson::new("2.3", "Do - Ti and Do - La", vec!["2.1", "2.2"]),
            EarMasterLesson::new("2.4", "Do - So", vec!["1.12"]),
            EarMasterLesson::new("2.5", "Do - Fa", vec!["1.12"]),
            EarMasterLesson::new("2.6", "Do - So and Do - Fa", vec!["2.4", "2.5"]),
            EarMasterLesson::new("2.7", "Do to Ti, La, So and Fa", vec!["2.3", "2.6"]),
            EarMasterLesson::new("2.8", "Do - Mi", vec!["2.7"]),
            EarMasterLesson::new("2.9", "Do - Re", vec!["2.7"]),
            EarMasterLesson::new("2.10", "Do - Do", vec!["2.7"]),
            EarMasterLesson::new("2.11", "Do to Mi, Re, and Do", vec!["2.8", "2.9", "2.10"]),
            EarMasterLesson::new("2.12", "All descending intervals from Do", vec!["2.11"]),
            // Unit 3 - Complementary intervals from Do - Ascending and descending
            EarMasterLesson::new("3.1", "Do - Re", vec!["2.12"]),
            EarMasterLesson::new("3.2", "Do - Mi", vec!["2.12"]),
            EarMasterLesson::new("3.3", "Do - Re and Do - Mi", vec!["3.1", "3.2"]),
            EarMasterLesson::new("3.4", "Do - Fa", vec!["2.12"]),
            EarMasterLesson::new("3.5", "Do - So", vec!["2.12"]),
            EarMasterLesson::new("3.6", "Do - Fa and Do - So", vec!["3.4", "3.5"]),
            EarMasterLesson::new("3.7", "Do to Re, Mi, Fa, and So", vec!["3.3", "3.6"]),
            EarMasterLesson::new("3.8", "Do - La", vec!["3.7"]),
            EarMasterLesson::new("3.9", "Do - Ti", vec!["3.7"]),
            EarMasterLesson::new("3.10", "Do - Do", vec!["3.7"]),
            EarMasterLesson::new("3.11", "Do to La, Ti and Do", vec!["3.8", "3.9", "3.10"]),
            EarMasterLesson::new("3.12", "All intervals from Do", vec!["3.11"]),
        ],
    };
    earmaster_course.course_builder()
}
