use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum PlayerTurn {
	Waiting,
	Playing
}
impl Default for PlayerTurn {
    fn default() -> PlayerTurn {
        PlayerTurn::Waiting
    }
}