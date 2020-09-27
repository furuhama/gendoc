use anyhow::Result;
use gendoc::*;

fn main() -> Result<()> {
    let mut option = parser::parse_option()?;

    option.convert();

    generator::generate(&option)?;

    Ok(())
}
