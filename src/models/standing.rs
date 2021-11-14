use serde::{Deserialize, Serialize};

#[derive( Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Standing {	
	Up,
	Down,
	Stunned
}