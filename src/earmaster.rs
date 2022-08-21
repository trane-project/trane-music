//! Module containing EarMaster courses.
pub mod chord_identification;
pub mod chord_inversions;
pub mod chord_progressions;
pub mod interval_identification;
pub mod interval_singing;
pub mod melody_dictation;
pub mod melody_sight_singing;
pub mod melody_singback;
pub mod rhythm_clapback;
pub mod rhythm_dictation;
pub mod rhythm_error_detection;
pub mod rhythm_sight_reading;
pub mod scale_identification;

use std::collections::BTreeMap;

use indoc::{formatdoc, indoc};
use trane::{
    course_builder::{AssetBuilder, CourseBuilder, ExerciseBuilder, LessonBuilder},
    data::{
        BasicAsset, CourseManifest, ExerciseAsset, ExerciseManifestBuilder, ExerciseType,
        LessonManifestBuilder,
    },
};
use ustr::Ustr;

use crate::AUTHORS;

/// Represents a lesson in EarMaster. Each lesson contains only one exercise corresponding to the
/// Earmaster unit with the given ID.
struct EarMasterLesson {
    /// A shorthand for for the ID of the lesson "1.1".
    id: Ustr,

    /// The full name of the lesson,
    name: String,

    /// The dependencies of this lesson, also written in the short ID format.
    dependencies: Vec<Ustr>,
}

impl EarMasterLesson {
    fn new(id: &str, name: &str, dependencies: Vec<&str>) -> Self {
        Self {
            id: Ustr::from(id),
            name: name.to_string(),
            dependencies: dependencies.iter().map(|&id| Ustr::from(id)).collect(),
        }
    }

    /// Generates a LessonBuilder based on this object.
    fn lesson_builder(&self, course_id: &Ustr, course_name: &str) -> LessonBuilder {
        let lesson_id = format!("{}::{}", course_id, self.id);
        let exercise_id = format!("{}::exercise", lesson_id);
        let dependencies: Vec<Ustr> = self
            .dependencies
            .iter()
            .map(|id| Ustr::from(&format!("{}::{}", course_id, id)))
            .collect();

        let lesson_id_clone = lesson_id.clone();
        let name_clone = self.name.clone();
        LessonBuilder {
            directory_name: format!("lesson_{}", self.id),
            asset_builders: vec![],
            manifest_closure: Box::new(move |m| {
                #[allow(clippy::redundant_clone)]
                m.clone()
                    .id(lesson_id_clone.clone())
                    .name(name_clone.clone())
                    .dependencies(dependencies.clone())
                    .clone()
            }),
            exercise_manifest_template: ExerciseManifestBuilder::default()
                .id(exercise_id)
                .lesson_id(lesson_id)
                .course_id(*course_id)
                .name(self.name.clone())
                .exercise_type(ExerciseType::Procedural)
                .exercise_asset(ExerciseAsset::FlashcardAsset {
                    front_path: "front.md".to_string(),
                    back_path: "back.md".to_string(),
                })
                .clone(),
            exercise_builders: vec![ExerciseBuilder {
                directory_name: "exercise".to_string(),
                manifest_closure: Box::new(|m| m),
                asset_builders: vec![
                    AssetBuilder {
                        file_name: "front.md".to_string(),
                        contents: formatdoc! {"
                            Work on the following exercise in EarMaster:
                            - Activity: {}
                            - Exercise Name: {}
                            - Exercise Number: {}
                        ", course_name, self.name, self.id},
                    },
                    AssetBuilder {
                        file_name: "back.md".to_string(),
                        contents: indoc! {"
                            Check your work on Earmaster and score this exercise accordingly.
                        "}
                        .to_string(),
                    },
                ],
            }],
        }
    }
}
/// Represents a course based on one of the activities in EarMaster.
struct EarMasterCourse {
    /// The full ID for this course.
    id: Ustr,

    /// The name of the course.
    name: String,

    /// The name of the directory under which the course is stored.
    directory_name: String,

    /// Optional metadata. The key value pair ("earmaster", "true") is added to every course.
    metadata: Option<BTreeMap<String, Vec<String>>>,

    /// The lessons in the course.
    lessons: Vec<EarMasterLesson>,
}

impl EarMasterCourse {
    fn course_builder(&self) -> CourseBuilder {
        let mut metadata: BTreeMap<String, Vec<String>> =
            BTreeMap::from([("earmaster".to_string(), vec!["true".to_string()])]);
        if let Some(input_metadata) = self.metadata.clone() {
            metadata.extend(input_metadata.into_iter());
        }

        let lesson_builders = self
            .lessons
            .iter()
            .map(|lesson| lesson.lesson_builder(&self.id, &self.name))
            .collect();

        CourseBuilder {
            directory_name: self.directory_name.clone(),
            course_manifest: CourseManifest {
                id: self.id,
                name: self.name.clone(),
                description: Some(format!("Practice EarMaster activity {}", self.name)),
                dependencies: vec![],
                authors: Some(vec![AUTHORS.to_string()]),
                metadata: Some(metadata),
                course_instructions: Some(BasicAsset::MarkdownAsset {
                    path: "instructions.md".to_string(),
                }),
                course_material: None,
            },
            lesson_manifest_template: LessonManifestBuilder::default().course_id(self.id).clone(),
            lesson_builders,
            asset_builders: vec![AssetBuilder {
                file_name: "instructions.md".to_string(),
                contents: formatdoc! {"
                        This course contains the exercises from the {} 
                        activity in EarMaster 7.2. The exercises are referenced by the same
                        numbers as in EarMaster. If you do not have a copy of EarMaster, you
                        can add this course to the blacklist.
                    ", self.name},
            }],
        }
    }
}
