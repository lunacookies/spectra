use crate::palette::Palette;
use mottle::theme::ThemeBuilder;

pub(crate) fn add_rules(builder: &mut ThemeBuilder, palette: Palette) {
    builder.add_workspace_rule("foreground", palette.foreground());
    builder.add_workspace_rule("editor.foreground", palette.foreground());
    builder.add_workspace_rule("editor.background", palette.background());
}
