fn inject(mod_id: &str, button: usize) {
    if mod_id == "better_ping_display" && button == 0 {
        horizon_client_render::better_ping_display::toggle_server_ping();
    }
}
