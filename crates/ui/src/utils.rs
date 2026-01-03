//! Small, dependency-free helpers.
//!
//! This module intentionally avoids pulling in heavy crates (e.g. `uuid`) to keep WASM small.

use std::cell::Cell;

thread_local! {
    static NEXT_ID: Cell<u32> = Cell::new(0);
}

/// Generate a DOM-safe unique-ish id for the current runtime session.
///
/// This is used for associating `<label for="...">` with form controls without depending on
/// `uuid` (which adds noticeable WASM size).
pub fn next_dom_id(prefix: &str) -> String {
    NEXT_ID.with(|counter| {
        let id = counter.get();
        counter.set(id.wrapping_add(1));
        format!("{prefix}{id}")
    })
}


