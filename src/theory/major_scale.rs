use std::collections::BTreeMap;

use anyhow::Result;
use indoc::formatdoc;
use trane::{
    course_builder::{
        music::{
            circle_fifths::CircleFifthsCourse,
            notes::*,
            scales::{ScaleNotes, ScaleType},
            MusicMetadata,
        },
        AssetBuilder, CourseBuilder, ExerciseBuilder, LessonBuilder, TraneMetadata,
    },
    data::{
        CourseManifest, ExerciseAsset, ExerciseManifestBuilder, ExerciseType, LessonManifestBuilder,
    },
};

use crate::AUTHORS;

pub static COURSE_ID: &str = "trane::music::theory::major_scale";

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
                        COURSE_ID,
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

pub fn course_builder() -> Result<CourseBuilder> {
    let course_generator = CircleFifthsCourse {
        directory_name: "major_scale".to_string(),
        course_manifest: CourseManifest {
            id: COURSE_ID.to_string(),
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
            .course_id(COURSE_ID.to_string())
            .clone(),
        lesson_builder_generator: Box::new(|note, previous_note| {
            let lesson_id = format!("{}::{}", COURSE_ID, note.to_string());

            Ok(LessonBuilder {
                directory_name: format!("lesson_{}", note.to_ascii_string()),
                exercise_manifest_template: ExerciseManifestBuilder::default()
                    .course_id(COURSE_ID.to_string())
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
                        None => vec![],
                        Some(previous_note) => {
                            let dep_id = format!("{}::{}", COURSE_ID, previous_note.to_string());
                            vec![dep_id]
                        }
                    };

                    #[allow(clippy::redundant_clone)]
                    m.clone()
                        .id(format!("{}::{}", COURSE_ID, note.to_string()))
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
        extra_lesson_builders: vec![],
    };
    course_generator.generate_course_builder()
}
