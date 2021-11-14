use std::{fs, path::Path};
use serde::{Deserialize, Serialize};

use super::{player_status::PlayerStatus, race::Race, skill::Skill, square::Square, team::Team};

#[derive(Default, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Player {
    race: Option<Race>,
	title: String,
	cost: i64,
	ma: i64,
	st: i64,
	ag: i64,
	av: i64,
	number: i64,
	skills: Option<Vec<Skill>>,
	player_status: Option<PlayerStatus>,
	position: Option<Square>,
	team: Option<Team>,
}

impl Player {
	pub fn new() -> Player {
        Default::default()
    }

	pub fn deserialize(&mut self, model: &str) {
		let file_name = String::from(model);
		let file_path = Path::new(file_name.as_str());

    	let data = fs::read_to_string(file_path).expect("Unable to read file");
		
		let v: Player = serde_json::from_str(&data).unwrap();
		self.title = v.title;
		self.cost = v.cost;
		self.ma = v.ma;
		self.st = v.st;
		self.ag = v.ag;
		self.av = v.av;
		self.number = v.number;
    }

	pub fn serialize(&self) {
		let serialized = serde_json::to_string(self).unwrap();
    	println!("serialized = {}", serialized);
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{player::Player};

    #[test]
    fn serialize_works() {
        let player = Player::new();
        player.serialize();        
    }

	#[test]
    fn build_human_works() {
        let mut player = Player::new();

        player.deserialize("/home/oscar/work/rust_game/bb/src/data/ff/humans/human_blitzer.json");

		//TODO assert_eq!(player.race., "HUMANS");
		assert_eq!(player.title, "Blitzer");
		assert_eq!(player.cost, 90000);
		assert_eq!(player.ma, 7);
		assert_eq!(player.st, 3);
		assert_eq!(player.ag, 3);
		assert_eq!(player.av, 8);

		player.deserialize("/home/oscar/work/rust_game/bb/src/data/ff/humans/human_lineman.json");
		assert_eq!(player.title, "Lineman");
		assert_eq!(player.cost, 50000);
		assert_eq!(player.ma, 6);
		assert_eq!(player.st, 3);
		assert_eq!(player.ag, 3);
		assert_eq!(player.av, 8);

		player.deserialize("/home/oscar/work/rust_game/bb/src/data/ff/humans/human_catcher.json");
		assert_eq!(player.title, "Catcher");
		assert_eq!(player.cost, 70000);
		assert_eq!(player.ma, 8);
		assert_eq!(player.st, 2);
		assert_eq!(player.ag, 3);
		assert_eq!(player.av, 7);

		player.deserialize("/home/oscar/work/rust_game/bb/src/data/ff/humans/human_thrower.json");
		assert_eq!(player.title, "Thrower");
		assert_eq!(player.cost, 70000);
		assert_eq!(player.ma, 6);
		assert_eq!(player.st, 3);
		assert_eq!(player.ag, 3);
		assert_eq!(player.av, 8);
    }
}