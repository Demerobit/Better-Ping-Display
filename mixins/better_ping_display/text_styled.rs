// OVERWRITE on HudBatch::text_styled — shifts server list player count left if ping text expanded.
// Consumes CURRENT_ROW_SHIFT immediately so only the first text_styled call per row (player count)
// is shifted — MOTD lines on other rows are not affected.
fn inject(
    batch: &mut HudBatch,
    atlas: &HudAtlas,
    text: &crate::text::Text,
    x: f32,
    y: f32,
    color: u32,
    shadow: bool,
) -> f32 {
    let shift = crate::better_ping_display::CURRENT_ROW_SHIFT.with(|c| {
        let s = c.get();
        c.set(0.0); // consume immediately so only the player-count call is shifted
        s
    });
    batch.styled_text(atlas, text, x - shift, y, color, shadow, false, None)
}
