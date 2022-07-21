use std::collections::BTreeMap;

use indoc::indoc;
use lazy_static::lazy_static;
use trane::{
    course_builder::{
        music::MusicMetadata, AssetBuilder, CourseBuilder, ExerciseBuilder, LessonBuilder,
        TraneMetadata,
    },
    data::{
        CourseManifest, ExerciseAsset, ExerciseManifestBuilder, ExerciseType, LessonManifestBuilder,
    },
};
use ustr::Ustr;

use crate::{guitar::basic_guitar_fretboard, AUTHORS};

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::music::guitar::advanced_fretboard");
    pub static ref LESSON1_ID: Ustr =
        Ustr::from("trane::music::guitar::advanced_fretboard::lesson_1");
    pub static ref LESSON2_ID: Ustr =
        Ustr::from("trane::music::guitar::advanced_fretboard::lesson_2");
    pub static ref LESSON3_ID: Ustr =
        Ustr::from("trane::music::guitar::advanced_fretboard::lesson_3");
    pub static ref LESSON4_ID: Ustr =
        Ustr::from("trane::music::guitar::advanced_fretboard::lesson_4");
}

pub fn course_builder() -> CourseBuilder {
    let lesson1: LessonBuilder = LessonBuilder {
        directory_name: "lesson_1".to_string(),
        asset_builders: vec![],
        exercise_manifest_template: ExerciseManifestBuilder::default()
            .lesson_id(*LESSON1_ID)
            .course_id(*COURSE_ID)
            .exercise_type(ExerciseType::Procedural)
            .exercise_asset(ExerciseAsset::FlashcardAsset {
                front_path: "front.md".to_string(),
                back_path: "back.md".to_string(),
            })
            .clone(),
        manifest_closure: Box::new(|m| {
            #[allow(clippy::redundant_clone)]
            m.clone()
                .id(*LESSON1_ID)
                .dependencies(vec![])
                .name("Lesson 1".to_string())
                .description(Some(
                    "Find the location of two random notes in the fretboard".to_string(),
                ))
                .clone()
        }),
        exercise_builders: vec![ExerciseBuilder {
            directory_name: "exercise_1".to_string(),
            manifest_closure: Box::new(move |m| {
                #[allow(clippy::redundant_clone)]
                m.clone()
                    .id(format!("{}::exercise_1", *LESSON1_ID))
                    .name("Exercise 1".to_string())
                    .clone()
            }),
            asset_builders: vec![
                AssetBuilder {
                    file_name: "front.md".to_string(),
                    contents: indoc! {"
                        Pick two random notes. For each note, go up and down each string and
                        find the given note in the first twelve frets.

                        Perform this at 40 bpm using a metronome. Only set a high score in this
                        exercise when you can perform well with any random pair of notes.

                        This exercise was adopted from exercise 4 in this video: https://www.youtube.com/watch?v=PJddQ6Q0UDo
                    "}
                    .to_string(),
                },
                AssetBuilder {
                    file_name: "back.md".to_string(),
                    contents: indoc! {"
                        If you find yourself needing to look up the location of individual
                        notes, you should set a low score on this exercise and continue with
                        the previous exercises.
                    "}
                    .to_string(),
                },
            ],
        }],
    };

    let lesson2: LessonBuilder = LessonBuilder {
        directory_name: "lesson_2".to_string(),
        asset_builders: vec![],
        exercise_manifest_template: ExerciseManifestBuilder::default()
            .lesson_id(*LESSON2_ID)
            .course_id(*COURSE_ID)
            .exercise_type(ExerciseType::Procedural)
            .exercise_asset(ExerciseAsset::FlashcardAsset {
                front_path: "front.md".to_string(),
                back_path: "back.md".to_string(),
            })
            .clone(),
        manifest_closure: Box::new(|m| {
            #[allow(clippy::redundant_clone)]
            m.clone()
                .id(*LESSON2_ID)
                .dependencies(vec![*LESSON1_ID])
                .name("Lesson 2".to_string())
                .description(Some(
                    "Find the location of seven random notes in the fretboard".to_string(),
                ))
                .clone()
        }),
        exercise_builders: vec![ExerciseBuilder {
            directory_name: "exercise_1".to_string(),
            manifest_closure: Box::new(move |m| {
                #[allow(clippy::redundant_clone)]
                m.clone()
                    .id(format!("{}::exercise_1", *LESSON2_ID))
                    .name("Exercise 1".to_string())
                    .clone()
            }),
            asset_builders: vec![
                AssetBuilder {
                    file_name: "front.md".to_string(),
                    contents: indoc! {"
                        Pick seven random notes. For each note, go up and down each string and
                        find the given note in the first twelve frets.

                        Perform this at 40 bpm using a metronome. Only set a high score in this
                        exercise when you can perform well with any random sequence of notes.

                        This exercise was adopted from exercise 5 in this video: https://www.youtube.com/watch?v=PJddQ6Q0UDo
                    "}
                    .to_string(),
                },
                AssetBuilder {
                    file_name: "back.md".to_string(),
                    contents: indoc! {"
                        If you find yourself needing to look up the location of individual
                        notes, you should set a low score on this exercise and continue with
                        the previous exercises.
                    "}
                    .to_string(),
                },
            ],
        }],
    };

    let lesson3: LessonBuilder = LessonBuilder {
        directory_name: "lesson_3".to_string(),
        asset_builders: vec![],
        exercise_manifest_template: ExerciseManifestBuilder::default()
            .lesson_id(*LESSON3_ID)
            .course_id(*COURSE_ID)
            .exercise_type(ExerciseType::Procedural)
            .exercise_asset(ExerciseAsset::FlashcardAsset {
                front_path: "front.md".to_string(),
                back_path: "back.md".to_string(),
            })
            .clone(),
        manifest_closure: Box::new(|m| {
            #[allow(clippy::redundant_clone)]
            m.clone()
                .id(*LESSON3_ID)
                .dependencies(vec![*LESSON2_ID])
                .name("Lesson 3".to_string())
                .description(Some(
                    "Find the location of two random notes in the fretboard".to_string(),
                ))
                .clone()
        }),
        exercise_builders: vec![ExerciseBuilder {
            directory_name: "exercise_1".to_string(),
            manifest_closure: Box::new(move |m| {
                #[allow(clippy::redundant_clone)]
                m.clone()
                    .id(format!("{}::exercise_1", *LESSON3_ID))
                    .name("Exercise 1".to_string())
                    .clone()
            }),
            asset_builders: vec![
                AssetBuilder {
                    file_name: "front.md".to_string(),
                    contents: indoc! {"
                        Pick two random notes. For each note, go up and down each string and
                        find the given note in the first twelve frets.

                        Gradually increase the speed until you can do it at 80 bpm using a
                        metronome. Only set a high score in this exercise when you can perform
                        well with any random pair of notes.

                        This exercise was adopted from exercise 5 in this video: https://www.youtube.com/watch?v=PJddQ6Q0UDo
                    "}
                    .to_string(),
                },
                AssetBuilder {
                    file_name: "back.md".to_string(),
                    contents: indoc! {"
                        If you find yourself needing to look up the location of individual
                        notes, you should set a low score on this exercise and continue with
                        the previous exercises.
                    "}
                    .to_string(),
                },
            ],
        }],
    };

    let lesson4: LessonBuilder = LessonBuilder {
        directory_name: "lesson_4".to_string(),
        asset_builders: vec![],
        exercise_manifest_template: ExerciseManifestBuilder::default()
            .lesson_id(*LESSON4_ID)
            .course_id(*COURSE_ID)
            .exercise_type(ExerciseType::Procedural)
            .exercise_asset(ExerciseAsset::FlashcardAsset {
                front_path: "front.md".to_string(),
                back_path: "back.md".to_string(),
            })
            .clone(),
        manifest_closure: Box::new(|m| {
            #[allow(clippy::redundant_clone)]
            m.clone()
                .id(*LESSON4_ID)
                .dependencies(vec![*LESSON3_ID])
                .name("Lesson 4".to_string())
                .description(Some(
                    "Find the location of seven random notes in the fretboard".to_string(),
                ))
                .clone()
        }),
        exercise_builders: vec![ExerciseBuilder {
            directory_name: "exercise_1".to_string(),
            manifest_closure: Box::new(move |m| {
                #[allow(clippy::redundant_clone)]
                m.clone()
                    .id(format!("{}::exercise_1", *LESSON4_ID))
                    .name("Exercise 1".to_string())
                    .clone()
            }),
            asset_builders: vec![
                AssetBuilder {
                    file_name: "front.md".to_string(),
                    contents: indoc! {"
                        Pick seven random notes. For each note, go up and down each string and
                        find the given note in the first twelve frets.

                        Gradually increase the speed until you can do it at 80 bpm using a
                        metronome. Only set a high score in this exercise when you can perform
                        well with any random sequence of notes.

                        This exercise was adopted from exercise 6 in this video: https://www.youtube.com/watch?v=PJddQ6Q0UDo
                    "}
                    .to_string(),
                },
                AssetBuilder {
                    file_name: "back.md".to_string(),
                    contents: indoc! {"
                        If you find yourself needing to look up the location of individual
                        notes, you should set a low score on this exercise and continue with
                        the previous exercises.
                    "}
                    .to_string(),
                },
            ],
        }],
    };

    CourseBuilder {
        directory_name: "advanced_guitar_fretboard".to_string(),
        course_manifest: CourseManifest {
            id: *COURSE_ID,
            name: "Advanced Guitar Fretboard".to_string(),
            dependencies: vec![*basic_guitar_fretboard::LESSON3_ID],
            description: Some("Learn the position of notes in the guitar frateboard".to_string()),
            authors: Some(vec![AUTHORS.to_string()]),
            metadata: Some(BTreeMap::from([
                (TraneMetadata::Skill.to_string(), vec!["music".to_string()]),
                (
                    MusicMetadata::Instrument.to_string(),
                    vec!["guitar".to_string()],
                ),
                (
                    MusicMetadata::MusicalSkill.to_string(),
                    vec!["fretboard".to_string()],
                ),
            ])),
            course_material: None,
            course_instructions: None,
        },
        asset_builders: vec![],
        lesson_builders: vec![lesson1, lesson2, lesson3, lesson4],
        lesson_manifest_template: LessonManifestBuilder::default()
            .course_id(*COURSE_ID)
            .clone(),
    }
}
