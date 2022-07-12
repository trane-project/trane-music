//! Code to generate all the music courses.
mod earmaster;
mod guitar;
mod theory;

use std::path::Path;

use anyhow::Result;

static AUTHORS: &str = "The Trane Project";

fn build_courses(library_root: &Path) -> Result<()> {
    let course_builders = vec![
        earmaster::interval_identification::course_builder(),
        guitar::basic_guitar_fretboard::course_builder(),
        guitar::advanced_guitar_fretboard::course_builder(),
        theory::major_scale::course_builder()?,
        theory::major_pentatonic_scale::course_builder()?,
        theory::minor_scale::course_builder()?,
        theory::minor_pentatonic_scale::course_builder()?,
    ];

    for course_builder in course_builders {
        course_builder.build(library_root)?;
        println!("Built {} course", course_builder.course_manifest.name);
    }
    Ok(())
}

fn main() -> Result<()> {
    let curr_dir = std::env::current_dir()?;
    build_courses(curr_dir.as_path())
}

#[cfg(test)]
mod tests {
    use trane::scheduler::ExerciseScheduler;

    use crate::build_courses;

    #[test]
    fn open_library() -> anyhow::Result<()> {
        let temp_dir = tempfile::TempDir::new()?;
        let library_root = &temp_dir.path().to_path_buf();
        build_courses(library_root)?;
        let trane = trane::Trane::new(library_root.to_str().unwrap())?;
        let batch = trane.get_exercise_batch(None)?;
        assert!(!batch.is_empty());
        Ok(())
    }
}
