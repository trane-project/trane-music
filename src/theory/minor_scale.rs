use anyhow::Result;
use lazy_static::lazy_static;
use trane::course_builder::{music::scales::ScaleType, CourseBuilder};
use ustr::Ustr;

use crate::theory::{major_scale, scale_course::ScaleCourse};

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::music::theory::minor_scale");
}

pub fn course_builder() -> Result<CourseBuilder> {
    let scale_course = ScaleCourse {
        course_id: *COURSE_ID,
        dependencies: vec![*major_scale::COURSE_ID],
        directory_name: "minor_scale".to_string(),
        scale: ScaleType::Minor,
        note_alias: Some(|note| note.relative_minor()),
    };
    scale_course.course_builder()
}
