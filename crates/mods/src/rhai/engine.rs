// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

use crate::rhai::{MAX_SCRIPT_MEMORY, MAX_SCRIPT_OPS, SCRIPT_WARN_OPS};
use rhai::{Engine, ImmutableString, LexError, Token};
use std::sync::Arc;
use sysinfo::Pid;
use tracing::{error, info, warn};

fn get_mem_usage() -> Option<usize> {
    let mut sys = sysinfo::System::new();
    sys.refresh_all();

    let pid = Pid::from(std::process::id() as usize);

    if let Some(process) = sys.process(pid) {
        return usize::try_from(process.memory()).ok();
    }

    None
}

#[expect(deprecated)]
pub fn get_default_engine(
    name: &str,
    is_mod_enabled: Box<dyn Fn(ImmutableString, ImmutableString, ImmutableString) -> Option<bool>>,
) -> Engine {
    let mut engine = Engine::new();

    let name: Arc<str> = Arc::from(name);

    engine.disable_symbol("eval");
    engine.disable_symbol("print");

    let name_clone = Arc::clone(&name);
    engine.on_progress(move |count| {
        if get_mem_usage().is_none_or(|mem| mem >= MAX_SCRIPT_MEMORY) {
            let msg = format!("Memory usage limit of {MAX_SCRIPT_MEMORY} reached");
            error!("Mod `{}`: {}", name_clone, &msg);
            return Some(msg.into());
        }

        if count % SCRIPT_WARN_OPS == 0 {
            warn!("Mod `{}`: has reached {} clock ticks", name_clone, count);
        }

        if count >= MAX_SCRIPT_OPS {
            let msg = format!("Script execution limit of {MAX_SCRIPT_OPS} reached");
            error!("Mod `{}`: {}", name_clone, &msg);
            return Some(msg.into());
        }

        None
    });

    let name_clone = Arc::clone(&name);
    engine.on_parse_token(move |token, _, _| {
        if get_mem_usage().is_none_or(|mem| mem >= MAX_SCRIPT_MEMORY) {
            let msg = format!("Memory usage limit of {MAX_SCRIPT_MEMORY} reached");
            error!("Mod `{}`: {}", name_clone, &msg);
            return Token::LexError(Box::from(LexError::Runtime("Out of memory".to_string())));
        }

        token
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

    engine.register_fn("is_mod_enabled", is_mod_enabled);

    engine
}
