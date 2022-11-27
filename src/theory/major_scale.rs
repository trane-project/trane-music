use anyhow::Result;
use lazy_static::lazy_static;
use trane::course_builder::{music::scales::ScaleType, CourseBuilder};
use ustr::Ustr;

use crate::theory::scale_course::ScaleCourse;

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::music::theory::major_scale");
}

pub fn course_builder() -> Result<CourseBuilder> {
    let scale_course = ScaleCourse {
        course_id: *COURSE_ID,
        dependencies: vec![],
        directory_name: "major_scale".to_string(),
        scale: ScaleType::Major,
        note_alias: None,
    };
    scale_course.course_builder()
}
