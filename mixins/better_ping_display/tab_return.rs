fn inject(value: Vec<GuiDraw>) -> Vec<GuiDraw> {
    let pings = crate::better_ping_display::TAB_PINGS.with(|cell| cell.borrow().clone());
    let info = crate::better_ping_display::TAB_INFO.with(|cell| *cell.borrow());

    if pings.is_empty() || info.slots <= 0 || info.cols <= 0 {
        return value;
    }

    // Look for slot fills: they have height 8 (y1 - y0 == 8)
    let mut vanilla_slot_width = 0i32;
    let mut vanilla_xxo = 0i32;
    for item in &value {
        if let GuiDraw::Fill { x0, y0, x1, y1, .. } = item {
            if y1 - y0 == 8 && *x1 > *x0 {
                vanilla_slot_width = x1 - x0;
                vanilla_xxo = *x0;
                break;
            }
        }
    }

    if vanilla_slot_width <= 0 {
        return value;
    }

    let cols = info.cols;
    let max_w = info.max_w;
    let sw = info.sw;

    // Vanilla ping sprite reservation is 13px (10px sprite + 3px padding).
    // Our required reservation is max_w + 3px padding (1px right, 2px left).
    let extra_per_col = (max_w + 3 - 13).max(0);
    let new_slot_width = (vanilla_slot_width + extra_per_col).min((sw - 50) / cols);

    let vanilla_total = vanilla_slot_width * cols + (cols - 1) * 5;
    let new_total = new_slot_width * cols + (cols - 1) * 5;
    let new_xxo = sw / 2 - new_total / 2;

    // Per-column layout calculations:
    let mut old_col_lefts = Vec::with_capacity(cols as usize);
    let mut new_col_lefts = Vec::with_capacity(cols as usize);
    let mut new_col_rights = Vec::with_capacity(cols as usize);
    let mut col_shifts = Vec::with_capacity(cols as usize);

    for c in 0..cols {
        let old_left = vanilla_xxo + c * vanilla_slot_width + c * 5;
        let new_left = new_xxo + c * new_slot_width + c * 5;
        let new_right = new_left + new_slot_width;
        let shift = new_left - old_left;

        old_col_lefts.push(old_left);
        new_col_lefts.push(new_left);
        new_col_rights.push(new_right);
        col_shifts.push(shift);
    }

    let mut ping_index = 0usize;
    let rows = info.rows;

    let mut result = Vec::with_capacity(value.len());

    for item in value {
        match item {
            // Main dark band(s) covering the table background:
            GuiDraw::Fill { x0, y0, x1, y1, color }
                if color == 0x8000_0000 && x1 - x0 >= vanilla_total - 2 =>
            {
                let cur_width = x1 - x0;
                let target_width = cur_width + (new_total - vanilla_total).max(0);
                result.push(GuiDraw::Fill {
                    x0: sw / 2 - target_width / 2 - 1,
                    y0,
                    x1: sw / 2 + target_width / 2 + 1,
                    y1,
                    color,
                });
            }
            // Slot background fill:
            GuiDraw::Fill { x0, y0, x1, y1, color }
                if y1 - y0 == 8 && x1 - x0 == vanilla_slot_width =>
            {
                let mut matched_c = None;
                for (c, &old_left) in old_col_lefts.iter().enumerate() {
                    if (x0 - old_left).abs() <= 1 {
                        matched_c = Some(c);
                        break;
                    }
                }
                if let Some(c) = matched_c {
                    result.push(GuiDraw::Fill {
                        x0: new_col_lefts[c],
                        y0,
                        x1: new_col_rights[c],
                        y1,
                        color,
                    });
                } else {
                    result.push(GuiDraw::Fill { x0, y0, x1, y1, color });
                }
            }
            // Ping sprite -> replace with right-aligned colored numeric ping text:
            GuiDraw::Sprite { ref sprite, x: _, y, .. } if sprite.starts_with("icon/ping_") => {
                let (ms, tw) = pings.get(ping_index).copied().unwrap_or((-1, 10));
                let c = (ping_index as i32 / rows).clamp(0, cols - 1) as usize;
                ping_index += 1;

                let text_x = new_col_rights[c] - 1 - tw;
                result.push(GuiDraw::Text {
                    text: crate::text::Text::literal(crate::better_ping_display::ping_text(ms)),
                    x: text_x,
                    y,
                    color: crate::better_ping_display::ping_color(ms),
                    shadow: true,
                });
            }
            // Text elements within a player slot:
            GuiDraw::Text { text, x, y, color, shadow } => {
                let mut shifted_x = x;
                for c in 0..cols as usize {
                    if x >= old_col_lefts[c] && x < old_col_lefts[c] + vanilla_slot_width {
                        shifted_x = x + col_shifts[c];
                        break;
                    }
                }
                result.push(GuiDraw::Text { text, x: shifted_x, y, color, shadow });
            }
            // Other fills (marks):
            GuiDraw::Fill { x0, y0, x1, y1, color } => {
                let mut shifted_x0 = x0;
                let mut shifted_x1 = x1;
                for c in 0..cols as usize {
                    if x0 >= old_col_lefts[c] && x0 < old_col_lefts[c] + vanilla_slot_width {
                        shifted_x0 = x0 + col_shifts[c];
                        shifted_x1 = x1 + col_shifts[c];
                        break;
                    }
                }
                result.push(GuiDraw::Fill { x0: shifted_x0, y0, x1: shifted_x1, y1, color });
            }
            // Other sprites (faces, hearts):
            GuiDraw::Sprite { sprite, x, y, w, h, region, color, blend } => {
                let mut shifted_x = x;
                for c in 0..cols as usize {
                    if x >= old_col_lefts[c] && x < old_col_lefts[c] + vanilla_slot_width {
                        shifted_x = x + col_shifts[c];
                        break;
                    }
                }
                result.push(GuiDraw::Sprite { sprite, x: shifted_x, y, w, h, region, color, blend });
            }
        }
    }

    result
}
