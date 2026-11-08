// RETURN injection on draw_server_icon — map row y coordinate to ping ms.
fn inject(value: (), slot: usize, y: f32) {
    let _ = value;
    let ms = horizon_client_render::better_ping_display::SERVER_ROW_PINGS.with(|cell| {
        cell.borrow().get(slot).copied().flatten()
    });
    if let Some(ms) = ms {
        horizon_client_render::better_ping_display::record_server_cy_ping(y, ms);
    }
}
