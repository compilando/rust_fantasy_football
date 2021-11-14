use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum PassRangeEnum {
	QuickPass, 
	ShortPass,
	LongPass,
	LongBomb,
	OutOfRange
}

#[derive(Default, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct PassRange {
	
	modifier: i32,
	name: String
	
}

impl PassRange {
	pub fn new() -> PassRange {
        Default::default()
    }    
}

