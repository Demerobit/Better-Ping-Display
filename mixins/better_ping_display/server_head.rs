// HEAD injection on Screen::draw — stash per-row ping ms values before the draw loop.
fn inject(screen_self: &Screen) {
    horizon_client_render::better_ping_display::clear_server_pings();
    if !horizon_client_render::better_ping_display::show_server_ping() {
        return;
    }
    if let Screen::List { servers, pings, .. } = screen_self {
        let pings = pings.lock().unwrap();
        let row_pings: Vec<Option<i32>> = servers
            .iter()
            .map(|server| match pings.get(&server.ip) {
                Some(Ping::Ok { ms, .. }) => Some(i32::try_from(*ms).unwrap_or(i32::MAX)),
                _ => None,
            })
            .collect();
        horizon_client_render::better_ping_display::SERVER_ROW_PINGS.with(|cell| {
            *cell.borrow_mut() = row_pings;
        });
    }
}
