mod earmaster;
mod guitar;
mod theory;

use anyhow::Result;

static AUTHORS: &str = "The Trane Project";

fn main() -> Result<()> {
    let curr_dir = std::env::current_dir()?;

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
        course_builder.build(&curr_dir)?;
        println!("Built {} course", course_builder.course_manifest.name);
    }
    Ok(())
}
