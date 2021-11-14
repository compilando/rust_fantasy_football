use serde::{Deserialize, Serialize};

#[derive( Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum FfWeather {
	Swelteringheat,
	Verysunny,
	Nice,
	Pouringrain,
	Blizzard
}
