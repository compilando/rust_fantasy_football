use serde::{Deserialize, Serialize};

use crate::models::{basegame::{BaseGame, BaseGameLifeCycle}, gamephase::GamePhase};

use super::timeline_base::BaseTimeLine;

#[derive(Default, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct TimelineAlternate {
    x: i32,
    pub(crate) y: i32,
}

impl BaseTimeLine for TimelineAlternate {
	fn evolve_game(mut game: BaseGame) -> GamePhase {

        let current_phase = game.current_phase();
        let current_player = game.current_player();
        let result = game.end_phase();

        Default::default()
    }    
}

