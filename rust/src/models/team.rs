use super::{piece::Piece, team_status::TeamStatus};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Team {
    players: Vec<Piece>,
	rerolls: i32,
	fan_factor: i32,
	assistant_coaches: i32,
	cheerleaders: i32,
	team_status: TeamStatus,
	team_name: String
}

impl Team {
	pub fn new(team_name: String) -> Team {
        Team {
            team_name,
            ..Default::default()
        }
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
        let team = Team::new(String::from("Unnamed"));

        team.serialize();        
    }
}