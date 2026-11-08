// Better Ping Display runtime module
use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, Ordering};

pub static SHOW_SERVER_PING: AtomicBool = AtomicBool::new(true);

thread_local! {
    pub static TAB_LATENCIES: RefCell<Vec<i32>> = RefCell::new(Vec::new());
    // (slot, ms) mapping computed in server_head
    pub static SERVER_ROW_PINGS: RefCell<Vec<Option<i32>>> = RefCell::new(Vec::new());
    // (cy, ms) mapping recorded per drawn server icon
    pub static SERVER_CY_PINGS: RefCell<Vec<(f32, i32)>> = RefCell::new(Vec::new());
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

pub fn record_server_cy_ping(cy: f32, ms: i32) {
    SERVER_CY_PINGS.with(|cell| {
        cell.borrow_mut().push((cy, ms));
    });
}

pub fn lookup_server_ping(cy: f32) -> i32 {
    SERVER_CY_PINGS.with(|cell| {
        for (stashed_cy, ms) in cell.borrow().iter() {
            if (*stashed_cy - cy).abs() < 2.0 {
                return *ms;
            }
        }
        -1
    })
}

pub fn clear_server_pings() {
    SERVER_ROW_PINGS.with(|cell| cell.borrow_mut().clear());
    SERVER_CY_PINGS.with(|cell| cell.borrow_mut().clear());
}
