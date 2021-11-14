use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum FfPassRangeEnum {
	QuickPass, 
	ShortPass,
	LongPass,
	LongBomb,
	OutOfRange
}

#[derive(Default, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct FfPassRange {
	
	modifier: i32,
	name: String
	
}

impl FfPassRange {
	pub fn new() -> FfPassRange {
        Default::default()
    }    
}

