mod imp;
mod palette;

use mottle::theme::ThemeBuilder;
use palette::Palette;
use std::io;

fn main() -> io::Result<()> {
    let mut light = ThemeBuilder::new("Spectra Light".to_string(), mottle::theme::Type::Light);
    imp::add_rules(&mut light, Palette::Light);

    let mut dark = ThemeBuilder::new("Spectra Dark".to_string(), mottle::theme::Type::Dark);
    imp::add_rules(&mut dark, Palette::Dark);

    light.build().save()?;
    dark.build().save()?;

    Ok(())
}
