use mottle::theme::ThemeBuilder;
use std::io;

fn main() -> io::Result<()> {
    let light = ThemeBuilder::new("Spectra Light".to_string(), mottle::theme::Type::Light).build();
    let dark = ThemeBuilder::new("Spectra Dark".to_string(), mottle::theme::Type::Dark).build();

    light.save()?;
    dark.save()?;

    Ok(())
}
