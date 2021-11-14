use std::collections::HashSet;
use serde::{Deserialize, Serialize};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Cell {
  pub x: i32,
  pub y: i32,
}

pub type CellSet = HashSet<Cell>;