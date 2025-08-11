// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2025 Crypts of the Lost Team

use serde::Deserialize;

#[derive(Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
pub struct Tiles {
    pub(crate) tiles: Vec<Tile>,
}

#[derive(Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
pub struct Tile {}
