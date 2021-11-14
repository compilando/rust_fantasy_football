use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum FfRace {
	Humans, 
    Orcs
}

impl Default for FfRace {
    fn default() -> FfRace {
        FfRace::Humans
    }
}
