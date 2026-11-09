// OVERWRITE on HudBatch::sprite_scaled — replace server list ping sprite with colored numeric text.
fn inject(
    batch: &mut HudBatch,
    atlas: &HudAtlas,
    name: &str,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    color: u32,
) {
    let is_server_ping = name.starts_with("gui/sprites/server_list/ping_")
        || name.starts_with("gui/sprites/server_list/pinging_");

    if is_server_ping && crate::better_ping_display::show_server_ping() {
        let ms = crate::better_ping_display::CURRENT_ROW_PING.with(|c| c.get());
        let text = crate::better_ping_display::ping_text(ms);
        let col = crate::better_ping_display::ping_color(ms);

        // Measure text width. The vanilla ping sprite is 10px wide.
        let tw = HudBatch::text_width(atlas, &text);
        // Right-align numeric ping at the right edge where the sprite was (x + 10).
        let text_x = x + 10.0 - tw;

        // If text is wider than the vanilla 10px sprite, record the extra shift
        // so text_styled can nudge the player count left by the same amount.
        let shift = (tw - 10.0).max(0.0);
        crate::better_ping_display::CURRENT_ROW_SHIFT.with(|c| c.set(shift));

        batch.text(atlas, &text, text_x, y + 1.0, col, true);
        return;
    }

    if let Some(r) = atlas.sprites.get(name).copied() {
        batch.rect_uv(x, y, w, h, r, color);
    }
}
