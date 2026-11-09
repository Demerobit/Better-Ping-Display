fn inject(value: Vec<GuiDraw>) -> Vec<GuiDraw> {
    let pings = crate::better_ping_display::TAB_PINGS.with(|cell| cell.borrow().clone());
    let info = crate::better_ping_display::TAB_INFO.with(|cell| *cell.borrow());

    if pings.is_empty() || info.slots <= 0 || info.cols <= 0 {
        return value;
    }

    // Detect vanilla slot width and left edge from the slot background fills (height == 8).
    let mut vanilla_slot_width = 0i32;
    let mut vanilla_xxo = 0i32;
    for item in &value {
        if let GuiDraw::Fill { x0, y0, x1, y1, .. } = item {
            if y1 - y0 == 8 && x1 > x0 {
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

    // Vanilla ping reservation is 13px (10px sprite + 3px gap).
    // Expand if the widest ping text needs more.
    let extra_per_col = (max_w + 3 - 13).max(0);
    let new_slot_width = (vanilla_slot_width + extra_per_col).min((sw - 50) / cols);

    let vanilla_total = vanilla_slot_width * cols + (cols - 1) * 5;
    let new_total = new_slot_width * cols + (cols - 1) * 5;
    let new_xxo = sw / 2 - new_total / 2;

    // Per-column x offsets.
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

    let cols_usize = cols as usize;
    let rows = info.rows;

    let shift_x = |x: i32| -> i32 {
        for c in 0..cols_usize {
            if x >= old_col_lefts[c] && x < old_col_lefts[c] + vanilla_slot_width {
                return x + col_shifts[c];
            }
        }
        x
    };

    let mut ping_index = 0usize;
    let mut result = Vec::with_capacity(value.len());

    for item in value {
        match item {
            // Wide dark band behind the whole table.
            GuiDraw::Fill { x0, y0, x1, y1, color }
                if color == 0x8000_0000 && x1 - x0 >= vanilla_total - 2 =>
            {
                let old_w = x1 - x0;
                let growth = new_total - vanilla_total;
                let new_w = old_w + growth.max(0);
                result.push(GuiDraw::Fill {
                    x0: sw / 2 - new_w / 2 - 1,
                    y0,
                    x1: sw / 2 + new_w / 2 + 1,
                    y1,
                    color,
                });
            }
            // Slot background (height exactly 8, width exactly one slot).
            GuiDraw::Fill { x0, y0, x1, y1, color }
                if y1 - y0 == 8 && x1 - x0 == vanilla_slot_width =>
            {
                let mut new_x0 = x0;
                let mut new_x1 = x1;
                for c in 0..cols_usize {
                    if (x0 - old_col_lefts[c]).abs() <= 1 {
                        new_x0 = new_col_lefts[c];
                        new_x1 = new_col_rights[c];
                        break;
                    }
                }
                result.push(GuiDraw::Fill { x0: new_x0, y0, x1: new_x1, y1, color });
            }
            // Ping sprite → replace with right-aligned colored numeric text.
            GuiDraw::Sprite { ref sprite, y, .. } if sprite.starts_with("icon/ping_") => {
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
            // All other items: shift x coordinate into the new column layout.
            GuiDraw::Text { text, x, y, color, shadow } => {
                result.push(GuiDraw::Text { text, x: shift_x(x), y, color, shadow });
            }
            GuiDraw::Fill { x0, y0, x1, y1, color } => {
                result.push(GuiDraw::Fill {
                    x0: shift_x(x0),
                    y0,
                    x1: shift_x(x1),
                    y1,
                    color,
                });
            }
            GuiDraw::Sprite { sprite, x, y, w, h, region, color, blend } => {
                result.push(GuiDraw::Sprite { sprite, x: shift_x(x), y, w, h, region, color, blend });
            }
            GuiDraw::AtlasSprite { sprite, x, y, w, h, color } => {
                result.push(GuiDraw::AtlasSprite { sprite, x: shift_x(x), y, w, h, color });
            }
            GuiDraw::Blit { texture, blend, x, y, w, h, uv, src, tex, color } => {
                result.push(GuiDraw::Blit { texture, blend, x: shift_x(x), y, w, h, uv, src, tex, color });
            }
            GuiDraw::Item { slot, x, y, seed, scale, pivot } => {
                result.push(GuiDraw::Item { slot, x: shift_x(x), y, seed, scale, pivot });
            }
            // Push/Pop: pass through unchanged (they don't have x/y positions).
            other => result.push(other),
        }
    }

    result
}
