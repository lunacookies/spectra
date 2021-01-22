use crate::palette::Palette;
use mottle::theme::Scope::*;
use mottle::theme::ThemeBuilder;

pub(crate) fn add_rules(builder: &mut ThemeBuilder, palette: Palette) {
    builder.add_workspace_rule("foreground", palette.foreground());
    builder.add_workspace_rule("editor.foreground", palette.foreground());
    builder.add_workspace_rule("editor.background", palette.background());

    builder.add_rule(Semantic("keyword"), palette.green());

    builder.add_rules(&[Semantic("function"), Semantic("method")], palette.blue());

    builder.add_rules(
        &[
            Semantic("type"),
            Semantic("class"),
            Semantic("struct"),
            Semantic("enum"),
            Semantic("enumMember"),
            Semantic("union"),
            Semantic("typeAlias"),
            Semantic("builtinType"),
            Semantic("typeParameter"),
        ],
        palette.yellow(),
    );

    builder.add_rules(
        &[Semantic("string"), Semantic("number"), Semantic("boolean")],
        palette.cyan(),
    );

    builder.add_rule(Semantic("property"), palette.blue());

    builder.add_rules(
        &[
            Semantic("macro"),
            Semantic("attribute"),
            Semantic("*.attribute"),
        ],
        palette.red(),
    );
}
