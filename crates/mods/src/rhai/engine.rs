// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

use crate::rhai::{MAX_SCRIPT_OPS, SCRIPT_WARN_OPS};
use rhai::{Engine, ImmutableString};
use std::sync::Arc;
use tracing::{error, info, warn};

pub fn get_default_engine(name: &str) -> Engine {
    let mut engine = Engine::new();

    let name: Arc<str> = Arc::from(name);

    engine.disable_symbol("eval");
    engine.disable_symbol("print");

    let name_clone = Arc::clone(&name);
    engine.on_progress(move |count| {
        if count % SCRIPT_WARN_OPS == 0 {
            warn!("Mod `{}`: {}%", name_clone, count);
        }

        if count >= MAX_SCRIPT_OPS {
            let msg = format!("Script execution limit of {MAX_SCRIPT_OPS} reached");
            error!("Mod `{}`: {}", name_clone, &msg);
            return Some(msg.into());
        }

        None
    });

    let name_clone = Arc::clone(&name);
    engine.register_fn("info", move |msg: ImmutableString| {
        info!("Mod `{}`: {msg}", name_clone);
    });

    let name_clone = Arc::clone(&name);
    engine.register_fn("warn", move |msg: ImmutableString| {
        warn!("Mod `{}`: {msg}", name_clone);
    });

    let name_clone = Arc::clone(&name);
    engine.register_fn("error", move |msg: ImmutableString| {
        error!("Mod `{}`: {msg}", name_clone);
    });

    engine
}
