use serde::{Deserialize, Serialize};

use crate::models::{basegame::{BaseGame, BaseGameLifeCycle}, gamephase::GamePhase};

#[derive(Default, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct TimelineAlternate {
    x: i32,
    y: i32,
}

impl TimelineAlternate {
	pub fn evolve_game(mut game: BaseGame) -> GamePhase {

        let result = game.end_phase();

        Default::default()
    }    
}

