//! A JSON-in, JSON-out wasm interface to `rsim::session` for the game client's Web Worker.
//!
//! The host writes a request into memory from `alloc`, calls `call(ptr, len)`, and reads the
//! response from `out_ptr()` (its length is `call`'s return value). A panic traps; the host then
//! reads its message from `panic_ptr()` / `panic_len()`.

use rsim::game::Pool;
use rsim::model::Model;
use rsim::session::{Config, Session};
use rsim::sigil::SigilDef;
use serde_json::{json, Value};
use std::cell::RefCell;

#[derive(Default)]
struct State {
    pool: Option<&'static Pool>,
    model: Option<&'static Model>,
    session: Option<Session>,
    out: Vec<u8>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
    // Separate from STATE, which a panic usually finds borrowed.
    static PANIC: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

#[no_mangle]
pub extern "C" fn alloc(len: usize) -> *mut u8 {
    let mut v = Vec::<u8>::with_capacity(len);
    let p = v.as_mut_ptr();
    std::mem::forget(v);
    p
}

/// # Safety
/// `p` and `len` must come from one `alloc` call.
#[no_mangle]
pub unsafe extern "C" fn dealloc(p: *mut u8, len: usize) {
    drop(Vec::from_raw_parts(p, 0, len));
}

#[no_mangle]
pub extern "C" fn out_ptr() -> *const u8 {
    STATE.with(|s| s.borrow().out.as_ptr())
}

#[no_mangle]
pub extern "C" fn panic_ptr() -> *const u8 {
    PANIC.with(|p| p.borrow().as_ptr())
}

#[no_mangle]
pub extern "C" fn panic_len() -> usize {
    PANIC.with(|p| p.borrow().len())
}

/// # Safety
/// `p` must point to `len` readable bytes of UTF-8 JSON.
#[no_mangle]
pub unsafe extern "C" fn call(p: *const u8, len: usize) -> usize {
    let input = std::slice::from_raw_parts(p, len);
    let resp = match serde_json::from_slice::<Value>(input) {
        Ok(req) => handle(&req),
        Err(e) => json!({"ok": false, "error": format!("bad request: {e}")}),
    };
    let bytes = resp.to_string().into_bytes();
    STATE.with(|s| {
        let mut s = s.borrow_mut();
        s.out = bytes;
        s.out.len()
    })
}

fn handle(req: &Value) -> Value {
    match req["op"].as_str().unwrap_or("") {
        "init" => init(req),
        "new" => {
            let cfg: Config = match serde_json::from_value(req["config"].clone()) {
                Ok(c) => c,
                Err(e) => return json!({"ok": false, "error": format!("bad config: {e}")}),
            };
            let (pool, model) = STATE.with(|s| (s.borrow().pool, s.borrow().model));
            let (Some(pool), Some(model)) = (pool, model) else {
                return json!({"ok": false, "error": "call init first"});
            };
            let session = Session::new(pool, model, cfg);
            STATE.with(|s| s.borrow_mut().session = Some(session));
            respond(req, Ok(()))
        }
        "act" => {
            let force = req["force"].as_bool().unwrap_or(false);
            let res = with_session(|s| s.act(&req["action"], force));
            match res {
                Some(r) => respond(req, r),
                None => json!({"ok": false, "error": "no game"}),
            }
        }
        "view" => respond(req, Ok(())),
        op => json!({"ok": false, "error": format!("unknown op {op}")}),
    }
}

fn init(req: &Value) -> Value {
    std::panic::set_hook(Box::new(|info| {
        let msg = info.to_string().into_bytes();
        PANIC.with(|p| *p.borrow_mut() = msg);
    }));
    let mut defs: Vec<SigilDef> = vec![];
    for d in req["defs"].as_array().into_iter().flatten() {
        match serde_json::from_value(d.clone()) {
            Ok(def) => defs.push(def),
            Err(e) => return json!({"ok": false, "error": format!("sigil {}: {e}", d["id"])}),
        }
    }
    let model: Model = match serde_json::from_value(req["model"].clone()) {
        Ok(m) => m,
        Err(e) => return json!({"ok": false, "error": format!("model: {e}")}),
    };
    let pool: &'static Pool = Box::leak(Box::new(Pool::new(defs)));
    let model: &'static Model = Box::leak(Box::new(model));
    STATE.with(|s| {
        let mut s = s.borrow_mut();
        s.pool = Some(pool);
        s.model = Some(model);
    });
    json!({"ok": true, "sigils": pool.defs.len()})
}

fn with_session<T>(f: impl FnOnce(&mut Session) -> T) -> Option<T> {
    STATE.with(|s| s.borrow_mut().session.as_mut().map(f))
}

fn respond(req: &Value, res: Result<(), String>) -> Value {
    let all = req["all"].as_bool().unwrap_or(false);
    with_session(|s| {
        json!({
            "ok": res.is_ok(),
            "error": res.err(),
            "view": s.view(all),
            "events": s.take_events(),
        })
    })
    .unwrap_or_else(|| json!({"ok": false, "error": "no game"}))
}
