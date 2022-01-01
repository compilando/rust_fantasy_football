use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct BaseBoard {

}

impl BaseBoard {
	pub fn new() -> BaseBoard {
        BaseBoard {
            
        }
    }
}
