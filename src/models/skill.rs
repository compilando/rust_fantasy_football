use serde::{Deserialize, Serialize};

#[derive( Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Skill{
	SureHands,
	Pass,
	Block,
	Catch,
	Dodge,
	None
}
