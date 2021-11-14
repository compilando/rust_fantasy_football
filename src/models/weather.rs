use serde::{Deserialize, Serialize};

#[derive( Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Weather {
	Swelteringheat,
	Verysunny,
	Nice,
	Pouringrain,
	Blizzard
}
