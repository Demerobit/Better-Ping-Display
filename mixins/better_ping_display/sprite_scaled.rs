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
        let ms = crate::better_ping_display::lookup_server_ping(y);
        let text = crate::better_ping_display::ping_text(ms);
        let col = crate::better_ping_display::ping_color(ms);
        batch.text(atlas, &text, x, y + 1.0, col, true);
        return;
    }

    if let Some(r) = atlas.sprites.get(name).copied() {
        batch.rect_uv(x, y, w, h, r, color);
    }
}
