use super::{team::Team};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Coach {
    name: String,
	team: Team
}

impl Coach {
	pub fn new() -> Coach {
        Default::default()
    }    
}

