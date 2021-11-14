use std::{fs, path::Path};
use serde::{Deserialize, Serialize};

use super::{geometry::square::Square, race::Race, skill::Skill, team::Team};

#[derive(Default, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Piece {
    race: Option<Race>,
	title: String,
	cost: i64,
	ma: i64,
	st: i64,
	ag: i64,
	av: i64,
	number: i64,
	skills: Option<Vec<Skill>>,
	position: Option<Square>,
	team: Option<Team>,
}

impl Piece {
	pub fn new() -> Piece {
        Default::default()
    }

	pub fn deserialize(&mut self, model: &str) {
		let file_name = String::from(model);
		let file_path = Path::new(file_name.as_str());

    	let data = fs::read_to_string(file_path).expect("Unable to read file");
		
		let v: Piece = serde_json::from_str(&data).unwrap();
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
    use crate::models::{piece::Piece};

    #[test]
    fn serialize_works() {
        let piece = Piece::new();
        piece.serialize();        
    }

	#[test]
    fn build_human_works() {
        let mut piece = Piece::new();

        piece.deserialize("/home/oscar/work/rust_game/bb/src/data/ff/humans/human_blitzer.json");

		//TODO assert_eq!(piece.race., "HUMANS");
		assert_eq!(piece.title, "Blitzer");
		assert_eq!(piece.cost, 90000);
		assert_eq!(piece.ma, 7);
		assert_eq!(piece.st, 3);
		assert_eq!(piece.ag, 3);
		assert_eq!(piece.av, 8);

		piece.deserialize("/home/oscar/work/rust_game/bb/src/data/ff/humans/human_lineman.json");
		assert_eq!(piece.title, "Lineman");
		assert_eq!(piece.cost, 50000);
		assert_eq!(piece.ma, 6);
		assert_eq!(piece.st, 3);
		assert_eq!(piece.ag, 3);
		assert_eq!(piece.av, 8);

		piece.deserialize("/home/oscar/work/rust_game/bb/src/data/ff/humans/human_catcher.json");
		assert_eq!(piece.title, "Catcher");
		assert_eq!(piece.cost, 70000);
		assert_eq!(piece.ma, 8);
		assert_eq!(piece.st, 2);
		assert_eq!(piece.ag, 3);
		assert_eq!(piece.av, 7);

		piece.deserialize("/home/oscar/work/rust_game/bb/src/data/ff/humans/human_thrower.json");
		assert_eq!(piece.title, "Thrower");
		assert_eq!(piece.cost, 70000);
		assert_eq!(piece.ma, 6);
		assert_eq!(piece.st, 3);
		assert_eq!(piece.ag, 3);
		assert_eq!(piece.av, 8);
    }
}