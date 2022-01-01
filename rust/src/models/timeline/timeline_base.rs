use crate::models::{basegame::BaseGame, gamephase::GamePhase};

pub trait BaseTimeLine {
    fn evolve_game(game: BaseGame) -> GamePhase;
}