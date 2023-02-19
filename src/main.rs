//! Code to generate all the music courses.
mod theory;

use std::path::Path;

use anyhow::Result;

static AUTHORS: &str = "The Trane Project";

fn build_courses(library_root: &Path) -> Result<()> {
    let course_builders = vec![
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
    use std::{fs, path::Path};

    use anyhow::Result;
    use trane::{course_library::CourseLibrary, scheduler::ExerciseScheduler, Trane};

    use crate::build_courses;

    #[test]
    fn verify_generated_courses() -> Result<()> {
        let temp_dir = tempfile::TempDir::new()?;
        let library_root = &temp_dir.path().to_path_buf();
        build_courses(library_root)?;
        let trane = Trane::new(library_root, library_root)?;
        let batch = trane.get_exercise_batch(None)?;
        assert!(!batch.is_empty());
        Ok(())
    }

    #[test]
    fn verify_courses() -> Result<()> {
        let courses_path = Path::new("courses");
        let trane = Trane::new(&std::env::current_dir()?, Path::new("courses"))?;
        assert!(trane.get_all_exercise_ids()?.len() > 0);
        fs::remove_dir_all(courses_path.join(".trane"))?;
        Ok(())
    }
}
