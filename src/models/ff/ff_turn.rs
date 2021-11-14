use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum FfPlayerTurn {
	Unused,
	MoveAction,
	PassAction,
	BlitzAction,
	HandOffAction,
	FoulAction,
	BlockAction,
	Used
}
impl Default for FfPlayerTurn {
    fn default() -> FfPlayerTurn {
        FfPlayerTurn::Unused
    }
}