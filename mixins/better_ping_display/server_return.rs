// RETURN injection on draw_server_icon — draw numeric ping text for this server row.
fn inject(value: (), atlas: &HudAtlas, batch: &mut HudBatch, slot: usize, x: f32, y: f32) {
    let _ = value;
    let ms = horizon_client_render::better_ping_display::SERVER_ROW_PINGS.with(|cell| {
        cell.borrow().get(slot).copied().flatten()
    });
    if let Some(ms) = ms {
        // icon_x is where the ping sprite is drawn: x + ROW_WIDTH - 4.0 - 15.0
        // We place the text just left of the sprite.
        let icon_x = x + ROW_WIDTH - 4.0 - 15.0;
        let text = horizon_client_render::better_ping_display::ping_text(ms);
        let color = horizon_client_render::better_ping_display::ping_color(ms);
        let tw = HudBatch::text_width(atlas, &text);
        batch.text(atlas, &text, icon_x - tw - 2.0, y + 1.0, color, true);
    }
}
