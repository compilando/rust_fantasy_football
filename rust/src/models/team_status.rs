use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct TeamStatus {
    score: i32,
	rerolls: i32,
	rerolled_this_turn: bool,
	fans: i32,
	fame: i32,
	babes: i32,
	has_blitzed: bool,
	has_fouled: bool,
	has_passed: bool,
	has_handed_of: bool
}

impl TeamStatus {

	fn new() -> TeamStatus {
        Default::default()
    }

    fn reset(&mut self) {
		self.rerolled_this_turn = false;
		self.has_blitzed = false;
		self.has_fouled = false;
		self.has_passed = false;
		self.has_handed_of = false;
	}
}

#[cfg(test)]
mod tests {
    use crate::models::team_status::TeamStatus;

    #[test]
    fn reset_works() {
        let mut team_status = TeamStatus::new();

		team_status.rerolled_this_turn = true;
		team_status.has_blitzed = true;
		team_status.has_fouled = true;
		team_status.has_passed = true;
		team_status.has_handed_of = true;
        team_status.reset();

        assert_eq!(team_status.rerolled_this_turn, false);
		assert_eq!(team_status.has_blitzed, false);
		assert_eq!(team_status.has_fouled, false);
		assert_eq!(team_status.has_passed, false);
		assert_eq!(team_status.has_handed_of, false);
    }
}