// Better Ping Display runtime module
use std::cell::{Cell, RefCell};
use std::sync::atomic::{AtomicBool, Ordering};

pub static SHOW_SERVER_PING: AtomicBool = AtomicBool::new(true);

#[derive(Clone, Copy, Debug, Default)]
pub struct TabLayoutInfo {
    pub max_w: i32,
    pub sw: i32,
    pub slots: i32,
    pub rows: i32,
    pub cols: i32,
}

thread_local! {
    // Tab list
    pub static TAB_PINGS: RefCell<Vec<(i32, i32)>> = RefCell::new(Vec::new());
    pub static TAB_INFO: RefCell<TabLayoutInfo> = RefCell::new(TabLayoutInfo::default());

    // Server list: per-server ping values from Screen::List
    pub static SERVER_ROW_PINGS: RefCell<Vec<i32>> = RefCell::new(Vec::new());
    // Current server ping being rendered
    pub static CURRENT_ROW_PING: Cell<i32> = Cell::new(-1);
    // Shift for player count on the current row (consumed immediately by text_styled)
    pub static CURRENT_ROW_SHIFT: Cell<f32> = Cell::new(0.0);
}

pub fn ping_color(ms: i32) -> u32 {
    match ms {
        i32::MIN..=-1 => 0xFFAAAAAA,
        0..=130 => 0xFF55FF55,
        131..=220 => 0xFFFFFF55,
        _ => 0xFFFF5555,
    }
}

pub fn ping_text(ms: i32) -> String {
    if ms < 0 {
        "?ms".to_owned()
    } else {
        format!("{}ms", ms)
    }
}

pub fn show_server_ping() -> bool {
    SHOW_SERVER_PING.load(Ordering::Relaxed)
}

pub fn toggle_server_ping() {
    let cur = SHOW_SERVER_PING.load(Ordering::Relaxed);
    SHOW_SERVER_PING.store(!cur, Ordering::Relaxed);
}

// ── Tab list sizing & layout info ───────────────────────────────────────────

pub fn set_tab_layout_info(
    pings: Vec<(i32, i32)>,
    max_w: i32,
    sw: i32,
    slots: i32,
    rows: i32,
    cols: i32,
) {
    TAB_PINGS.with(|cell| *cell.borrow_mut() = pings);
    TAB_INFO.with(|cell| {
        *cell.borrow_mut() = TabLayoutInfo {
            max_w,
            sw,
            slots,
            rows,
            cols,
        }
    });
}

// ── Server list cleanup ─────────────────────────────────────────────────────

pub fn clear_server_pings() {
    SERVER_ROW_PINGS.with(|cell| cell.borrow_mut().clear());
    CURRENT_ROW_PING.with(|cell| cell.set(-1));
    CURRENT_ROW_SHIFT.with(|cell| cell.set(0.0));
}
