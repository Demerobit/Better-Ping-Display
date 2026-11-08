fn inject(value: Vec<GuiDraw>) -> Vec<GuiDraw> {
    let latencies: Vec<i32> = crate::better_ping_display::TAB_LATENCIES.with(|cell| {
        cell.borrow().clone()
    });
    let mut ping_index = 0usize;
    value
        .into_iter()
        .map(|item| match item {
            GuiDraw::Sprite { ref sprite, x, y, .. } if sprite.starts_with("icon/ping_") => {
                let ms = latencies.get(ping_index).copied().unwrap_or(-1);
                ping_index += 1;
                GuiDraw::Text {
                    text: crate::text::Text::literal(crate::better_ping_display::ping_text(ms)),
                    x: x - 4,
                    y,
                    color: crate::better_ping_display::ping_color(ms),
                    shadow: true,
                }
            }
            other => other,
        })
        .collect()
}
