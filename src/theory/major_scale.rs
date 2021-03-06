use std::collections::BTreeMap;

use anyhow::Result;
use indoc::formatdoc;
use lazy_static::lazy_static;
use trane::{
    course_builder::{
        music::{circle_fifths::CircleFifthsCourse, notes::*, scales::ScaleType, MusicMetadata},
        AssetBuilder, CourseBuilder, ExerciseBuilder, LessonBuilder, TraneMetadata,
    },
    data::{
        CourseManifest, ExerciseAsset, ExerciseManifestBuilder, ExerciseType, LessonManifestBuilder,
    },
};
use ustr::Ustr;

use crate::AUTHORS;

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::music::theory::major_scale");
    pub static ref BASICS_LESSON_ID: Ustr = Ustr::from(&format!("{}::basics", *COURSE_ID));
}

fn generate_exercise_builders(note: Note) -> Result<Vec<ExerciseBuilder>> {
    let scale = ScaleType::Major.notes(note)?;
    let mut builders = vec![];
    for (index, scale_note) in scale.notes.into_iter().enumerate() {
        let degree = index + 1;
        builders.push(ExerciseBuilder {
            directory_name: format!("exercise_{}", degree),
            asset_builders: vec![
                AssetBuilder {
                    file_name: "front.md".to_string(),
                    contents: formatdoc! {"
                        What note is degree number {} of the {} major scale?
                    ", degree, note.to_string()}
                    .to_string(),
                },
                AssetBuilder {
                    file_name: "back.md".to_string(),
                    contents: formatdoc! {"
                        The note in degree number {} of the {} major scale is {}.
                    ", degree, note.to_string(), scale_note.to_string()}
                    .to_string(),
                },
            ],
            manifest_closure: Box::new(move |m| {
                #[allow(clippy::redundant_clone)]
                m.clone()
                    .id(format!(
                        "{}::{}::exercise_{}",
                        *COURSE_ID,
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
fn generate_basics_lesson() -> Result<LessonBuilder> {
    let intervals = ScaleType::Major.intervals()?;

    let mut exercises = vec![];
    for (index, interval) in intervals.iter().enumerate() {
        let degree = index + 1;
        let exercise = ExerciseBuilder {
            directory_name: format!("exercise_{}", degree),
            asset_builders: vec![
                AssetBuilder {
                    file_name: "front.md".to_string(),
                    contents: formatdoc! {"
                        What interval from the tonic is degree number {} of the
                        major scale?
                    ", degree}
                    .to_string(),
                },
                AssetBuilder {
                    file_name: "back.md".to_string(),
                    contents: formatdoc! {"
                        The interval from the tonic of degree number {} of the
                        major scale is the {}.
                    ", degree, interval.to_string()}
                    .to_string(),
                },
            ],
            manifest_closure: Box::new(move |m| {
                #[allow(clippy::redundant_clone)]
                m.clone()
                    .id(format!("{}::exercise_{}", *BASICS_LESSON_ID, degree))
                    .name(format!("Exercise {}", degree))
                    .clone()
            }),
        };
        exercises.push(exercise);
    }

    Ok(LessonBuilder {
        directory_name: "basics".to_string(),
        exercise_manifest_template: ExerciseManifestBuilder::default()
            .course_id(*COURSE_ID)
            .lesson_id(*BASICS_LESSON_ID)
            .exercise_type(ExerciseType::Declarative)
            .exercise_asset(ExerciseAsset::FlashcardAsset {
                front_path: "front.md".to_string(),
                back_path: "back.md".to_string(),
            })
            .clone(),
        asset_builders: vec![],
        exercise_builders: exercises,
        manifest_closure: Box::new(move |m| {
            #[allow(clippy::redundant_clone)]
            m.clone()
                .id(*BASICS_LESSON_ID)
                .name("Major Scale - Basic Construction".to_string())
                .description(Some(
                    "Learn the intervals which make up the major scale.".to_string(),
                ))
                .dependencies(vec![])
                .clone()
        }),
    })
}

pub fn course_builder() -> Result<CourseBuilder> {
    let course_generator = CircleFifthsCourse {
        directory_name: "major_scale".to_string(),
        course_manifest: CourseManifest {
            id: *COURSE_ID,
            name: "The Major Scale".to_string(),
            dependencies: vec![],
            description: Some("Learn the notes of the major scale for all twelve keys".to_string()),
            authors: Some(vec![AUTHORS.to_string()]),
            metadata: Some(BTreeMap::from([
                (TraneMetadata::Skill.to_string(), vec!["music".to_string()]),
                (
                    MusicMetadata::MusicalConcept.to_string(),
                    vec!["scales".to_string()],
                ),
                (
                    MusicMetadata::ScaleType.to_string(),
                    vec!["major".to_string()],
                ),
            ])),
            course_material: None,
            course_instructions: None,
        },
        course_asset_builders: vec![],
        note_alias: None,
        lesson_manifest_template: LessonManifestBuilder::default()
            .course_id(*COURSE_ID)
            .clone(),
        lesson_builder_generator: Box::new(|note, previous_note| {
            let lesson_id = format!("{}::{}", *COURSE_ID, note.to_string());

            Ok(LessonBuilder {
                directory_name: format!("lesson_{}", note.to_ascii_string()),
                exercise_manifest_template: ExerciseManifestBuilder::default()
                    .course_id(*COURSE_ID)
                    .lesson_id(lesson_id)
                    .exercise_type(ExerciseType::Procedural)
                    .exercise_asset(ExerciseAsset::FlashcardAsset {
                        front_path: "front.md".to_string(),
                        back_path: "back.md".to_string(),
                    })
                    .clone(),
                asset_builders: vec![],
                exercise_builders: generate_exercise_builders(note)?,
                manifest_closure: Box::new(move |m| {
                    let deps = match previous_note {
                        None => vec![Ustr::from(&format!("{}::basics", *COURSE_ID))],
                        Some(previous_note) => {
                            let dep_id = Ustr::from(&format!(
                                "{}::{}",
                                *COURSE_ID,
                                previous_note.to_string()
                            ));
                            vec![dep_id]
                        }
                    };

                    #[allow(clippy::redundant_clone)]
                    m.clone()
                        .id(format!("{}::{}", *COURSE_ID, note.to_string()))
                        .name(format! {"Major Scale - Key of {}", note.to_string()})
                        .description(Some(format!(
                            "Learn the notes of the major scale in the key of {}",
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
        extra_lessons_generator: Some(Box::new(|| Ok(vec![generate_basics_lesson()?]))),
    };
    course_generator.generate_course_builder()
}
