pub mod advanced_guitar_fretboard;
pub mod basic_guitar_fretboard;

use indoc::{formatdoc, indoc};
use trane::course_builder::{music::notes::*, AssetBuilder};

pub trait FretboardAnswer {
    fn get_answer(&self) -> String;
}

impl FretboardAnswer for Note {
    fn get_answer(&self) -> String {
        match *self {
            Note::A => indoc! {"
                - 1st string (high E): 5th fret
                - 2nd string (B): 10th fret
                - 3rd string (G): 2nd fret
                - 4th string (D): 7th fret
                - 5th string (A): 12th fret
                - 6th string (low E): 5th fret
            "}
            .to_string(),
            Note::A_FLAT => indoc! {"
                - 1st string (high E): 4th fret
                - 2nd string (B): 9th fret
                - 3rd string (G): 1st fret
                - 4th string (D): 6th fret
                - 5th string (A): 11th fret
                - 6th string (low E): 4th fret
            "}
            .to_string(),
            Note::A_SHARP => indoc! {"
                - 1st string (high E): 6th fret
                - 2nd string (B): 11th fret
                - 3rd string (G): 3rd fret
                - 4th string (D): 8th fret
                - 5th string (A): 1st fret
                - 6th string (low E): 6th fret
            "}
            .to_string(),
            Note::B => indoc! {"
                - 1st string (high E): 7th fret
                - 2nd string (B): 12th fret
                - 3rd string (G): 4th fret
                - 4th string (D): 9th fret
                - 5th string (A): 2nd fret
                - 6th string (low E): 7th fret
            "}
            .to_string(),
            Note::B_FLAT => indoc! {"
                - 1st string (high E): 6th fret
                - 2nd string (B): 11th fret
                - 3rd string (G): 3rd fret
                - 4th string (D): 8th fret
                - 5th string (A): 1st fret
                - 6th string (low E): 6th fret
            "}
            .to_string(),
            Note::B_SHARP => indoc! {"
                - 1st string (high E): 8th fret
                - 2nd string (B): 1st fret
                - 3rd string (G): 5th fret
                - 4th string (D): 10th fret
                - 5th string (A): 3rd fret
                - 6th string (low E): 8th fret
            "}
            .to_string(),
            Note::C => indoc! {"
                - 1st string (high E): 8th fret
                - 2nd string (B): 1st fret
                - 3rd string (G): 5th fret
                - 4th string (D): 10th fret
                - 5th string (A): 3rd fret
                - 6th string (low E): 8th fret
            "}
            .to_string(),
            Note::C_FLAT => indoc! {"
                - 1st string (high E): 7th fret
                - 2nd string (B): 12th fret
                - 3rd string (G): 4th fret
                - 4th string (D): 9th fret
                - 5th string (A): 2nd fret
                - 6th string (low E): 7th fret
            "}
            .to_string(),
            Note::C_SHARP => indoc! {"
                - 1st string (high E): 9th fret
                - 2nd string (B): 2nd fret
                - 3rd string (G): 6th fret
                - 4th string (D): 11th fret
                - 5th string (A): 4th fret
                - 6th string (low E): 9th fret
            "}
            .to_string(),
            Note::D => indoc! {"
                - 1st string (high E): 10th fret
                - 2nd string (B): 3rd fret
                - 3rd string (G): 7th fret
                - 4th string (D): 12th fret
                - 5th string (A): 5th fret
                - 6th string (low E): 10th fret
            "}
            .to_string(),
            Note::D_FLAT => indoc! {"
                - 1st string (high E): 9th fret
                - 2nd string (B): 2nd fret
                - 3rd string (G): 6th fret
                - 4th string (D): 11th fret
                - 5th string (A): 4th fret
                - 6th string (low E): 9th fret
            "}
            .to_string(),
            Note::D_SHARP => indoc! {"
                - 1st string (high E): 11th fret
                - 2nd string (B): 4th fret
                - 3rd string (G): 8th fret
                - 4th string (D): 1st fret
                - 5th string (A): 6th fret
                - 6th string (low E): 11th fret
            "}
            .to_string(),
            Note::E => indoc! {"
                - 1st string (high E): 12th fret
                - 2nd string (B): 5th fret
                - 3rd string (G): 9th fret
                - 4th string (D): 2nd fret
                - 5th string (A): 7th fret
                - 6th string (low E): 12th fret
            "}
            .to_string(),
            Note::E_FLAT => indoc! {"
                - 1st string (high E): 11th fret
                - 2nd string (B): 4th fret
                - 3rd string (G): 8th fret
                - 4th string (D): 1st fret
                - 5th string (A): 6th fret
                - 6th string (low E): 11th fret
            "}
            .to_string(),
            Note::E_SHARP => indoc! {"
                - 1st string (high E): 1st fret
                - 2nd string (B): 6th fret
                - 3rd string (G): 10th fret
                - 4th string (D): 3rd fret
                - 5th string (A): 8th fret
                - 6th string (low E): 1st fret
            "}
            .to_string(),
            Note::F => indoc! {"
                - 1st string (high E): 1st fret
                - 2nd string (B): 6th fret
                - 3rd string (G): 10th fret
                - 4th string (D): 3rd fret
                - 5th string (A): 8th fret
                - 6th string (low E): 1st fret
            "}
            .to_string(),
            Note::F_FLAT => indoc! {"
                - 1st string (high E): 12th fret
                - 2nd string (B): 5th fret
                - 3rd string (G): 9th fret
                - 4th string (D): 2nd fret
                - 5th string (A): 7th fret
                - 6th string (low E): 12th fret
            "}
            .to_string(),
            Note::F_SHARP => indoc! {"
                - 1st string (high E): 2nd fret
                - 2nd string (B): 7th fret
                - 3rd string (G): 11th fret
                - 4th string (D): 4th fret
                - 5th string (A): 9th fret
                - 6th string (low E): 2nd fret
            "}
            .to_string(),
            Note::G => indoc! {"
                - 1st string (high E): 3rd fret
                - 2nd string (B): 8th fret
                - 3rd string (G): 12th fret
                - 4th string (D): 5th fret
                - 5th string (A): 10th fret
                - 6th string (low E): 3rd fret
            "}
            .to_string(),
            Note::G_FLAT => indoc! {"
                - 1st string (high E): 2nd fret
                - 2nd string (B): 7th fret
                - 3rd string (G): 11th fret
                - 4th string (D): 4th fret
                - 5th string (A): 9th fret
                - 6th string (low E): 2nd fret
            "}
            .to_string(),
            Note::G_SHARP => indoc! {"
                - 1st string (high E): 4th fret
                - 2nd string (B): 9th fret
                - 3rd string (G): 1st fret
                - 4th string (D): 6th fret
                - 5th string (A): 11th fret
                - 6th string (low E): 4th fret
            "}
            .to_string(),
        }
    }
}

pub fn generate_asset_builders(note: Note, tempo: &str) -> Vec<AssetBuilder> {
    vec![
        AssetBuilder {
            file_name: "front.md".to_string(),
            contents: formatdoc! {"
                Find the note {} in the fretboard {}.
            ", note.to_string(), tempo},
        },
        AssetBuilder {
            file_name: "back.md".to_string(),
            contents: note.get_answer(),
        },
    ]
}
