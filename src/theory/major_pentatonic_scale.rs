use anyhow::Result;
use lazy_static::lazy_static;
use trane::{course_builder::CourseBuilder, data::music::scales::ScaleType};
use ustr::Ustr;

use crate::theory::{minor_scale, scale_course::ScaleCourse};

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::music::theory::major_pentatonic_scale");
}

pub fn course_builder() -> Result<CourseBuilder> {
    let scale_course = ScaleCourse {
        course_id: *COURSE_ID,
        dependencies: vec![*minor_scale::COURSE_ID],
        directory_name: "major_pentatonic_scale".to_string(),
        scale: ScaleType::MajorPentatonic,
        note_alias: None,
    };
    scale_course.course_builder()
}
