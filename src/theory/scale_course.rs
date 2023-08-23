use std::collections::BTreeMap;

use anyhow::Result;
use indoc::formatdoc;
use trane::{
    course_builder::{
        music::{circle_fifths::CircleFifthsCourse, MusicMetadata},
        AssetBuilder, CourseBuilder, ExerciseBuilder, LessonBuilder, TraneMetadata,
    },
    data::{
        music::{notes::*, scales::ScaleType},
        CourseManifest, ExerciseAsset, ExerciseManifestBuilder, ExerciseType,
        LessonManifestBuilder,
    },
};
use ustr::Ustr;

use crate::AUTHORS;

/// Generates a course that teaches the intervals and notes in the given scale.
pub struct ScaleCourse {
    /// The ID of the course.
    pub course_id: Ustr,

    /// The dependencies for this course.
    pub dependencies: Vec<Ustr>,

    /// The name of the directory under which the course will be stored.
    pub directory_name: String,

    /// The scale type that this course is about.
    pub scale: ScaleType,

    /// An optional function used to generate the name of the note for the lesson. Useful, for
    /// example, to generate a course on the minor scale which follows the circle of fifths.
    pub note_alias: Option<fn(Note) -> Result<Note>>,
}

impl ScaleCourse {
    /// Generates the ID of the lesson teaching the basic intervals of the scale.
    pub fn basics_lesson_id(course_id: Ustr) -> Ustr {
        Ustr::from(&format!("{}::basics", course_id))
    }

    /// Generates the exercise builders for the lesson with the given scale and note.
    fn generate_exercise_builders(
        course_id: Ustr,
        scale: ScaleType,
        note: Note,
    ) -> Result<Vec<ExerciseBuilder>> {
        let scale_notes = scale.notes(note)?;
        let mut builders = vec![];
        for (index, scale_note) in scale_notes.notes.into_iter().enumerate() {
            // Skip the first note, which is the tonic and is always the same as the note.
            if index == 0 {
                continue;
            }

            let degree = index + 1;
            builders.push(ExerciseBuilder {
                directory_name: format!("exercise_{}", degree),
                asset_builders: vec![
                    AssetBuilder {
                        file_name: "front.md".to_string(),
                        contents: formatdoc! {"
                            What note is degree number {} of the {} {} scale?
                        ", degree, note.to_string(), scale.to_string()}
                        .to_string(),
                    },
                    AssetBuilder {
                        file_name: "back.md".to_string(),
                        contents: formatdoc! {"
                            The note in degree number {} of the {} {} scale is {}.
                        ", degree, note.to_string(), scale.to_string(), scale_note.to_string()}
                        .to_string(),
                    },
                ],
                manifest_closure: Box::new(move |m| {
                    #[allow(clippy::redundant_clone)]
                    m.clone()
                        .id(format!(
                            "{}::{}::exercise_{}",
                            course_id,
                            note.to_string(),
                            degree
                        ))
                        .name(format!("Exercise {}", degree))
                        .clone()
                }),
            })
        }
        Ok(builders)
    }

    /// Generates a first lesson which teaches how the scale is constructed.
    fn generate_basics_lesson(course_id: Ustr, scale: ScaleType) -> Result<LessonBuilder> {
        let intervals = scale.intervals()?;
        let mut exercises = vec![];

        for (index, interval) in intervals.iter().enumerate() {
            // Skip the first interval, which is the tonic and is always a unison interval.
            if index == 0 {
                continue;
            }

            let degree = index + 1;
            let lesson_id = Self::basics_lesson_id(course_id);
            let exercise = ExerciseBuilder {
                directory_name: format!("exercise_{}", degree),
                asset_builders: vec![
                    AssetBuilder {
                        file_name: "front.md".to_string(),
                        contents: formatdoc! {"
                            What interval from the tonic is degree number {} of the {} scale?
                        ", degree, scale.to_string()}
                        .to_string(),
                    },
                    AssetBuilder {
                        file_name: "back.md".to_string(),
                        contents: formatdoc! {"
                            The interval from the tonic of degree number {} of the {} scale is
                            the {}.
                        ", degree, scale.to_string(), interval.to_string()}
                        .to_string(),
                    },
                ],
                manifest_closure: Box::new(move |m| {
                    #[allow(clippy::redundant_clone)]
                    m.clone()
                        .id(format!("{}::exercise_{}", lesson_id, degree))
                        .name(format!("Exercise {}", degree))
                        .clone()
                }),
            };
            exercises.push(exercise);
        }

        let lesson_id = Self::basics_lesson_id(course_id);
        Ok(LessonBuilder {
            directory_name: "basics".to_string(),
            exercise_manifest_template: ExerciseManifestBuilder::default()
                .course_id(course_id)
                .lesson_id(lesson_id)
                .exercise_type(ExerciseType::Declarative)
                .exercise_asset(ExerciseAsset::FlashcardAsset {
                    front_path: "front.md".to_string(),
                    back_path: Some("back.md".to_string()),
                })
                .clone(),
            asset_builders: vec![],
            exercise_builders: exercises,
            manifest_closure: Box::new(move |m| {
                #[allow(clippy::redundant_clone)]
                m.clone()
                    .id(lesson_id)
                    .name(format! {"{} Scale - Basic Construction", scale.to_string()})
                    .description(Some(format! {
                    "Learn the intervals which make up the {} scale.", scale.to_string()}))
                    .dependencies(vec![])
                    .clone()
            }),
        })
    }

    /// Generates the course builder for the scale course.
    pub fn course_builder(&self) -> Result<CourseBuilder> {
        let course_id = self.course_id;
        let scale = self.scale;

        let course_generator = CircleFifthsCourse {
            directory_name: self.directory_name.clone(),
            course_manifest: CourseManifest {
                id: self.course_id,
                name: format!("The {} Scale", scale.to_string()),
                dependencies: self.dependencies.clone(),
                description: Some(format!(
                    "Learn the notes of the {} scale for all keys",
                    scale.to_string()
                )),
                authors: Some(vec![AUTHORS.to_string()]),
                metadata: Some(BTreeMap::from([
                    (TraneMetadata::Skill.to_string(), vec!["music".to_string()]),
                    (
                        TraneMetadata::Skill.to_string(),
                        vec!["music_theory".to_string()],
                    ),
                    (
                        MusicMetadata::MusicalConcept.to_string(),
                        vec!["scales".to_string()],
                    ),
                    (
                        MusicMetadata::ScaleType.to_string(),
                        vec![scale.to_string().to_lowercase()],
                    ),
                ])),
                course_material: None,
                course_instructions: None,
                generator_config: None,
            },
            course_asset_builders: vec![],
            note_alias: self.note_alias,
            lesson_manifest_template: LessonManifestBuilder::default()
                .course_id(self.course_id)
                .clone(),
            lesson_builder_generator: Box::new(move |note, previous_note| {
                let lesson_id = format!("{}::{}", course_id, note.to_string());

                Ok(LessonBuilder {
                    directory_name: format!("lesson_{}", note.to_ascii_string()),
                    exercise_manifest_template: ExerciseManifestBuilder::default()
                        .course_id(course_id)
                        .lesson_id(lesson_id)
                        .exercise_type(ExerciseType::Procedural)
                        .exercise_asset(ExerciseAsset::FlashcardAsset {
                            front_path: "front.md".to_string(),
                            back_path: Some("back.md".to_string()),
                        })
                        .clone(),
                    asset_builders: vec![],
                    exercise_builders: Self::generate_exercise_builders(course_id, scale, note)?,
                    manifest_closure: Box::new(move |m| {
                        let deps = match previous_note {
                            None => vec![Self::basics_lesson_id(course_id)],
                            Some(previous_note) => {
                                let dep_id = Ustr::from(&format!(
                                    "{}::{}",
                                    course_id,
                                    previous_note.to_string()
                                ));
                                vec![dep_id]
                            }
                        };

                        #[allow(clippy::redundant_clone)]
                        m.clone()
                            .id(format!("{}::{}", course_id, note.to_string()))
                            .name(format!(
                                "{} Scale - Key of {}",
                                scale.to_string(),
                                note.to_string()
                            ))
                            .description(Some(format!(
                                "Learn the notes of the {} scale in the key of {}",
                                scale.to_string(),
                                note.to_string()
                            )))
                            .dependencies(deps)
                            .metadata(Some(BTreeMap::from([(
                                MusicMetadata::Key.to_string(),
                                vec![note.to_ascii_string()],
                            )])))
                            .clone()
                    }),
                })
            }),
            extra_lessons_generator: Some(Box::new(move || {
                Ok(vec![Self::generate_basics_lesson(course_id, scale)?])
            })),
        };
        course_generator.generate_course_builder()
    }
}
