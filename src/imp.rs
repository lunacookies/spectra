use crate::palette::Palette;
use mottle::theme::Scope::*;
use mottle::theme::ThemeBuilder;

pub(crate) fn add_rules(builder: &mut ThemeBuilder, palette: Palette) {
    workspace_colors(builder, palette);
    syntax_highlighting(builder, palette);
}

fn workspace_colors(builder: &mut ThemeBuilder, palette: Palette) {
    builder.add_workspace_rule("foreground", palette.foreground());
    builder.add_workspace_rule("editor.foreground", palette.foreground());
    builder.add_workspace_rule("editor.background", palette.background());

    builder.add_workspace_rule("editorLineNumber.foreground", palette.deemphasized());
    builder.add_workspace_rule("editorGutter.background", palette.background_highlights());

    builder.add_workspace_rule(
        "rust_analyzer.inlayHints.foreground",
        palette.deemphasized(),
    );
}

fn syntax_highlighting(builder: &mut ThemeBuilder, palette: Palette) {
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
            Semantic("interface"),
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

    builder.add_rules(
        &[Semantic("formatSpecifier"), Semantic("escapeSequence")],
        palette.red(),
    );

    builder.add_rule(Semantic("property"), palette.blue());

    builder.add_rule(Semantic("namespace"), palette.cyan());

    builder.add_rules(
        &[
            Semantic("macro"),
            Semantic("attribute"),
            Semantic("*.attribute"),
        ],
        palette.red(),
    );

    builder.add_rule(Semantic("lifetime"), palette.magenta());

    builder.add_rule(Semantic("comment"), palette.deemphasized());
}
