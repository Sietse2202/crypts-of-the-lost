// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

use rhai::Scope;

pub fn get_default_scope<'a>() -> Scope<'a> {
    let mut scope = Scope::new();
    scope.push_constant("GAME_VERSION", env!("CARGO_PKG_VERSION"));
    scope
}
