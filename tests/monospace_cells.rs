//! With the `monospace_cells` feature and a monospace width set on the
//! buffer, every glyph must land exactly on the cell grid, even for a
//! proportional font. This is what lets terminals use any font.
#![cfg(feature = "monospace_cells")]

use std::path::PathBuf;

use cosmic_text::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping};

const CELL: f32 = 10.0;

fn layout(text: &str) -> Vec<(String, f32, f32, f32)> {
    let repo_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut font_system = FontSystem::new();
    font_system.db_mut().load_font_data(
        std::fs::read(PathBuf::from(&repo_dir).join("fonts/Inter-Regular.ttf")).unwrap(),
    );

    let mut buffer = Buffer::new(&mut font_system, Metrics::new(16.0, 20.0));
    let mut buffer = buffer.borrow_with(&mut font_system);
    buffer.set_monospace_width(Some(CELL));
    buffer.set_text(
        text,
        &Attrs::new().family(Family::Name("Inter")),
        Shaping::Advanced,
        None,
    );
    buffer.shape_until_scroll(true);

    buffer
        .layout_runs()
        .flat_map(|run| {
            run.glyphs
                .iter()
                .map(|g| (text[g.start..g.end].to_string(), g.x, g.w, g.x_offset))
                .collect::<Vec<_>>()
        })
        .collect()
}

#[test]
fn proportional_font_snaps_to_cells() {
    // Very narrow and very wide letters in a proportional font
    let glyphs = layout("iiWWmlI1 .");
    assert_eq!(glyphs.len(), 10);
    for (i, (text, x, w, _)) in glyphs.iter().enumerate() {
        assert_eq!(*w, CELL, "glyph {text:?} should be one cell wide");
        assert_eq!(
            *x,
            i as f32 * CELL,
            "glyph {text:?} should start at cell {i}"
        );
    }
}

#[test]
fn narrow_glyphs_are_centered() {
    let glyphs = layout("i");
    let (_, _, _, x_offset) = &glyphs[0];
    assert!(
        *x_offset > 0.0,
        "narrow glyph should be shifted right to center it"
    );
}
