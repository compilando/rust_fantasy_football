use super::passrange::{ PassRangeEnum};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct RangeRuler {
    quick_pass: i32,
    short_pass: i32,
    long_pass: i32,
    long_bomb: i32
}

impl RangeRuler {
	pub fn new() -> RangeRuler {
        RangeRuler {
            quick_pass: 3,
            short_pass: 7,
            long_pass: 10,
            long_bomb: 13
        }
    }    

    pub fn get_pass_range(self, distance: i32) -> PassRangeEnum {
		
        let rr = RangeRuler::new();

        if distance > rr.long_bomb {
			return PassRangeEnum::OutOfRange;
		} else if distance > rr.long_pass {
			return PassRangeEnum::LongBomb;
		} else if distance > rr.short_pass {
			return PassRangeEnum::LongPass;
		} else if distance > rr.quick_pass {
			return PassRangeEnum::ShortPass;
		} else {
			return PassRangeEnum::QuickPass;
		}		
	}
}

#[cfg(test)]
mod tests {
    use crate::models::{passrange::PassRangeEnum, rangeruler::RangeRuler};

    #[test]
    fn reset_get_pass_range() {
        let r_r = RangeRuler::new();

        assert_eq!(r_r.get_pass_range(2), PassRangeEnum::QuickPass);
    }
}
