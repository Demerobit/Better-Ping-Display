fn inject(tab_i: &TabInputs<'_>, font: &Font) {
    let sorted = crate::tab_list::sorted_players(tab_i.players, tab_i.board);
    let slots = sorted.len() as i32;
    let mut rows = slots;
    let mut cols = 1i32;
    while rows > crate::tab_list::MAX_ROWS_PER_COL {
        cols += 1;
        rows = (slots + cols - 1) / cols;
    }
    let mut pings = Vec::with_capacity(sorted.len());
    let mut max_w = 0i32;
    for p in &sorted {
        let text = crate::better_ping_display::ping_text(p.latency);
        let tw = font.width(&text).ceil() as i32;
        max_w = max_w.max(tw);
        pings.push((p.latency, tw));
    }
    crate::better_ping_display::set_tab_layout_info(
        pings,
        max_w,
        tab_i.screen_width,
        slots,
        rows,
        cols,
    );
}
