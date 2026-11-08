fn inject(tab_i: &TabInputs<'_>) {
    let sorted = crate::tab_list::sorted_players(tab_i.players, tab_i.board);
    crate::better_ping_display::TAB_LATENCIES.with(|cell| {
        let mut v = cell.borrow_mut();
        v.clear();
        for p in &sorted {
            v.push(p.latency);
        }
    });
}
