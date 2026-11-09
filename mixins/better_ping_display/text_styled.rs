// OVERWRITE on HudBatch::text_styled — shifts server list player count left if ping text expanded.
fn inject(
    batch: &mut HudBatch,
    atlas: &HudAtlas,
    text: &crate::text::Text,
    x: f32,
    y: f32,
    color: u32,
    shadow: bool,
) -> f32 {
    let shift = crate::better_ping_display::lookup_server_shift(y);
    batch.styled_text(atlas, text, x - shift, y, color, shadow, false, None)
}
