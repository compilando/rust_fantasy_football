use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Cointoss {
    tossed: bool,
	away_picked_heads: bool,
	result_heads: bool,
	home_receives: bool
	
}

impl Cointoss {
	pub fn new() -> Cointoss {
        Default::default()
    }    
}

