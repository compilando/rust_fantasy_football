use super::team_status::TeamStatus;
use super::player::Player;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Team {
    players: Vec<Player>,
	rerolls: i32,
	fan_factor: i32,
	assistant_coaches: i32,
	cheerleaders: i32,
	team_status: TeamStatus,
	team_name: String
}

impl Team {
	pub fn new() -> Team {
        Default::default()
    }    

	pub fn serialize(&self) {
		let serialized = serde_json::to_string(self).unwrap();
    	println!("serialized = {}", serialized);
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{team::Team};

    #[test]
    fn serialize_works() {
        let mut team = Team::new();

        team.serialize();        
    }
}