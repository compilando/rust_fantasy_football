use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Race {
	Humans, 
    Orcs
}

impl Default for Race {
    fn default() -> Race {
        Race::Humans
    }
}
