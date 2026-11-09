// HEAD injection on Screen::draw — stash per-row ping ms values before the draw loop.
fn inject(screen_self: &Screen) {
    horizon_client_render::better_ping_display::clear_server_pings();
    if !horizon_client_render::better_ping_display::show_server_ping() {
        return;
    }
    if let Screen::List { servers, pings, .. } = screen_self {
        let pings = pings.lock().unwrap();
        let mut row_pings = Vec::with_capacity(servers.len());
        for s in servers {
            let ping = match pings.get(&s.ip) {
                Some(Ping::Ok { ms, .. }) => Some(*ms as i32),
                Some(_) => Some(-1),
                None => None,
            };
            row_pings.push(ping);
        }
        horizon_client_render::better_ping_display::SERVER_ROW_PINGS.with(|cell| {
            *cell.borrow_mut() = row_pings;
        });
    }
}
