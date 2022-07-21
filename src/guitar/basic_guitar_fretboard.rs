use std::collections::BTreeMap;

use indoc::indoc;
use lazy_static::lazy_static;
use trane::{
    course_builder::{
        music::{notes::*, MusicMetadata},
        AssetBuilder, CourseBuilder, ExerciseBuilder, LessonBuilder, TraneMetadata,
    },
    data::{
        BasicAsset, CourseManifest, ExerciseAsset, ExerciseManifestBuilder, ExerciseType,
        LessonManifestBuilder,
    },
};
use ustr::Ustr;

use crate::{guitar::generate_asset_builders, AUTHORS};

lazy_static! {
    pub static ref COURSE_ID: Ustr = Ustr::from("trane::music::guitar::basic_fretboard");
    pub static ref LESSON1_ID: Ustr = Ustr::from("trane::music::guitar::basic_fretboard::lesson_1");
    pub static ref LESSON2_ID: Ustr = Ustr::from("trane::music::guitar::basic_fretboard::lesson_2");
    pub static ref LESSON3_ID: Ustr = Ustr::from("trane::music::guitar::basic_fretboard::lesson_3");
    pub static ref LESSON4_ID: Ustr = Ustr::from("trane::music::guitar::basic_fretboard::lesson_4");
}

static LESSON1_TEMPO: &str = "at a slow tempo without a metronome";
static LESSON2_TEMPO: &str = "at 40 bpm using a metronome";
static LESSON3_TEMPO: &str = "at 40 bpm using a metronome";
static LESSON4_TEMPO: &str = "gradually increasing the speed to 80 bpm using a metronome";

fn generate_exercise_builders(
    exercises: Vec<(usize, Note)>,
    lesson_id: Ustr,
    tempo: &str,
) -> Vec<ExerciseBuilder> {
    exercises
        .into_iter()
        .map(|(number, note)| ExerciseBuilder {
            directory_name: format!("exercise_{}", number),
            manifest_closure: Box::new(move |m| {
                #[allow(clippy::redundant_clone)]
                m.clone()
                    .id(format!("{}::exercise_{}", lesson_id, number))
                    .name(format!("Exercise {}", number))
                    .clone()
            }),
            asset_builders: generate_asset_builders(note, tempo),
        })
        .collect()
}

pub fn course_builder() -> CourseBuilder {
    let lesson1: LessonBuilder = LessonBuilder {
        directory_name: "lesson_1".to_string(),
        asset_builders: vec![AssetBuilder {
            file_name: "instructions.md".to_string(),
            contents: indoc! {"
                Go down each string and find the given note in the first twelve frets.
                Repeat another time going up the strings.

                Do this at a slow tempo but without a metronome.

                This exercise was adopted from exercise 1 in this video: https://www.youtube.com/watch?v=PJddQ6Q0UDo
            "}
            .to_string(),
        }],
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
                .lesson_instructions(Some(BasicAsset::MarkdownAsset {
                    path: "instructions.md".to_string(),
                }))
                .description(Some(
                    "Learn the position of all natural notes in the fretboard".to_string(),
                ))
                .clone()
        }),
        exercise_builders: generate_exercise_builders(
            vec![
                (1, Note::A),
                (2, Note::B),
                (3, Note::C),
                (4, Note::D),
                (5, Note::E),
                (6, Note::F),
                (7, Note::G),
            ],
            *LESSON1_ID,
            LESSON1_TEMPO,
        ),
    };

    let lesson2: LessonBuilder = LessonBuilder {
        directory_name: "lesson_2".to_string(),
        asset_builders: vec![AssetBuilder {
            file_name: "instructions.md".to_string(),
            contents: indoc! {"
                Go down each string and find the given note in the first twelve frets.
                Repeat another time going up the strings.

                Do this at 40 bpm using a metronome.

                This exercise was adopted from exercise 2 in this video: https://www.youtube.com/watch?v=PJddQ6Q0UDo
            "}
            .to_string(),
        }],
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
                .lesson_instructions(Some(BasicAsset::MarkdownAsset {
                    path: "instructions.md".to_string(),
                }))
                .description(Some(
                    "Practice the position of all natural notes in the fretboard at 40 bpm"
                        .to_string(),
                ))
                .clone()
        }),
        exercise_builders: generate_exercise_builders(
            vec![
                (1, Note::A),
                (2, Note::B),
                (3, Note::C),
                (4, Note::D),
                (5, Note::E),
                (6, Note::F),
                (7, Note::G),
            ],
            *LESSON2_ID,
            LESSON2_TEMPO,
        ),
    };

    let lesson3: LessonBuilder = LessonBuilder {
        directory_name: "lesson_3".to_string(),
        asset_builders: vec![AssetBuilder {
            file_name: "instructions.md".to_string(),
            contents: indoc! {"
                Go down each string and find the given note in the first twelve frets.
                Repeat another time going up the strings.

                Do this at 40 bpm using a metronome.

                This exercise was adopted from exercise 3 in this video: https://www.youtube.com/watch?v=PJddQ6Q0UDo
            "}
            .to_string(),
        }],
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
                .lesson_instructions(Some(BasicAsset::MarkdownAsset {
                    path: "instructions.md".to_string(),
                }))
                .description(Some(
                    "Practice the position of all accidental notes in the fretboard at 40 bpm"
                        .to_string(),
                ))
                .clone()
        }),
        exercise_builders: generate_exercise_builders(
            vec![
                (1, Note::A_FLAT),
                (2, Note::A_SHARP),
                (3, Note::B_FLAT),
                (4, Note::B_SHARP),
                (5, Note::C_FLAT),
                (6, Note::C_SHARP),
                (7, Note::D_FLAT),
                (8, Note::D_SHARP),
                (9, Note::E_FLAT),
                (10, Note::E_SHARP),
                (11, Note::F_FLAT),
                (12, Note::F_SHARP),
                (13, Note::G_FLAT),
                (14, Note::G_SHARP),
            ],
            *LESSON3_ID,
            LESSON3_TEMPO,
        ),
    };

    let lesson4: LessonBuilder = LessonBuilder {
        directory_name: "lesson_4".to_string(),
        asset_builders: vec![AssetBuilder {
            file_name: "instructions.md".to_string(),
            contents: indoc! {"
                Go down each string and find the given note in the first twelve frets.
                Repeat another time going up the strings.

                Do this gradually increasing the speed to 80 bpm using a metronome.

                This exercise was adopted from exercise 6 in this video: https://www.youtube.com/watch?v=PJddQ6Q0UDo
            "}
            .to_string(),
        }],
        exercise_manifest_template: ExerciseManifestBuilder::default()
            .lesson_id(LESSON4_ID.to_string())
            .course_id(COURSE_ID.to_string())
            .exercise_type(ExerciseType::Procedural)
            .exercise_asset(ExerciseAsset::FlashcardAsset {
                front_path: "front.md".to_string(),
                back_path: "back.md".to_string(),
            })
            .clone(),
        manifest_closure: Box::new(|m| {
            #[allow(clippy::redundant_clone)]
            m.clone()
                .id(LESSON4_ID.to_string())
                .dependencies(vec![*LESSON3_ID])
                .name("Lesson 4".to_string())
                .lesson_instructions(Some(BasicAsset::MarkdownAsset {
                    path: "instructions.md".to_string(),
                }))
                .description(Some(
                    "Practice the position of all notes in the fretboard up to 80 bpm".to_string(),
                ))
                .clone()
        }),
        exercise_builders: generate_exercise_builders(
            vec![
                (1, Note::A),
                (2, Note::A_FLAT),
                (3, Note::A_SHARP),
                (4, Note::B),
                (5, Note::B_FLAT),
                (6, Note::B_SHARP),
                (7, Note::C),
                (8, Note::C_FLAT),
                (9, Note::C_SHARP),
                (10, Note::D),
                (11, Note::D_FLAT),
                (12, Note::D_SHARP),
                (13, Note::E),
                (14, Note::E_FLAT),
                (15, Note::E_SHARP),
                (16, Note::F),
                (17, Note::F_FLAT),
                (18, Note::F_SHARP),
                (19, Note::G),
                (20, Note::G_FLAT),
                (21, Note::G_SHARP),
            ],
            *LESSON4_ID,
            LESSON4_TEMPO,
        ),
    };

    CourseBuilder {
        directory_name: "basic_guitar_fretboard".to_string(),
        course_manifest: CourseManifest {
            id: *COURSE_ID,
            name: "Basic Guitar Fretboard".to_string(),
            dependencies: vec![],
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
