// RETURN injection on draw_server_icon — set current row ping and reset shift.
fn inject(value: (), slot: usize, y: f32) {
    let _ = (value, y);
    let ms = horizon_client_render::better_ping_display::SERVER_ROW_PINGS.with(|cell| {
        cell.borrow().get(slot).copied().unwrap_or(-1)
    });
    horizon_client_render::better_ping_display::CURRENT_ROW_PING.with(|cell| {
        cell.set(ms);
    });
    horizon_client_render::better_ping_display::CURRENT_ROW_SHIFT.with(|cell| {
        cell.set(0.0);
    });
}
