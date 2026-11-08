fn inject(mod_id: &str, buttons: &mut Vec<String>) {
    if mod_id == "better_ping_display" {
        let label = if horizon_client_render::better_ping_display::show_server_ping() {
            "Server List Ping: ON"
        } else {
            "Server List Ping: OFF"
        };
        buttons.push(label.to_owned());
    }
}
