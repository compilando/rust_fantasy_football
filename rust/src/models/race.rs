use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Race {
	None, 
}

impl Default for Race {
    fn default() -> Race {
        Race::None
    }
}