use super::{ff_passrange::FfPassRangeEnum};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct FfRangeRuler {
    quick_pass: i32,
    short_pass: i32,
    long_pass: i32,
    long_bomb: i32
}

impl FfRangeRuler {
	pub fn new() -> FfRangeRuler {
        FfRangeRuler {
            quick_pass: 3,
            short_pass: 7,
            long_pass: 10,
            long_bomb: 13
        }
    }    

    pub fn get_pass_range(self, distance: i32) -> FfPassRangeEnum {
		
        let rr = FfRangeRuler::new();

        if distance > rr.long_bomb {
			return FfPassRangeEnum::OutOfRange;
		} else if distance > rr.long_pass {
			return FfPassRangeEnum::LongBomb;
		} else if distance > rr.short_pass {
			return FfPassRangeEnum::LongPass;
		} else if distance > rr.quick_pass {
			return FfPassRangeEnum::ShortPass;
		} else {
			return FfPassRangeEnum::QuickPass;
		}		
	}
}

#[cfg(test)]
mod tests {
    use crate::models::ff::{ff_passrange::FfPassRangeEnum, ff_rangeruler::FfRangeRuler};


    #[test]
    fn reset_get_pass_range() {
        let r_r = FfRangeRuler::new();

        assert_eq!(r_r.get_pass_range(2), FfPassRangeEnum::QuickPass);
    }
}
