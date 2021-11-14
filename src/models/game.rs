use super::{gameerror::GameError, team::Team};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Game {
    team_a: Option<Team>,
    team_b: Option<Team>,
    phases: Vec<super::gamephase::GamePhase>,
    current_phase: usize
}

impl Game {
	pub fn new() -> Game {
        Game {
            phases: Vec::new(),
            team_a: None,
            team_b: None,
            current_phase: 0
        }
    }

    fn set_phases(&mut self, phases: Vec<super::gamephase::GamePhase>) {
        for item in phases {
            let _ = &self.phases.push(item);
        }
    }

    fn phases_count(&mut self) -> usize {
        self.phases.len()
    }
    

    pub fn end_phase(&mut self) -> Result<usize, GameError> {
        let phase_len = self.phases.len();

        log::error!("phase_len {:?}", phase_len);
        if self.current_phase > phase_len {
            let _msg = format!("The game is already finished");
            Err(GameError::GameAlreadyFinished)
        }
        else if self.current_phase == phase_len {
            self.current_phase += 1;
            Ok(phase_len)
        }
        else {
            self.current_phase += 1;
            Ok(self.current_phase)
        }
    }

    pub fn start(&mut self) -> Result<(), GameError> {
        
        let phases_count = self.phases_count();
        if phases_count == 0 {
            return Err(GameError::PhasesNotDefined);
        }

        if self.team_a.is_none() || self.team_b.is_none() {
            return Err(GameError::TeamsNotDefined);
        }

        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use crate::models::{game::Game, gamephase::GamePhase};

    #[test]
    fn new_game_works() {
        let mut game = Game::new();

        let part_1 = GamePhase::new(String::from("Primera parte"), 8);
        let part_intermedio = GamePhase::new(String::from("Descanso"), 0);
        let part_2 = GamePhase::new(String::from("Segunda parte"), 8);

        game.set_phases(vec![part_1, part_intermedio, part_2]);

        assert_eq!(game.phases_count(), 3);        
    }

    #[test]
    fn end_phase_past_works() {
        let mut game = Game::new();

        let part_1 = GamePhase::new(String::from("Primera parte"), 8);
        let part_intermedio = GamePhase::new(String::from("Descanso"), 0);
        let part_2 = GamePhase::new(String::from("Segunda parte"), 8);

        game.set_phases(vec![part_1, part_intermedio, part_2]);
        assert_eq!(game.phases_count(), 3);

        let start_res = game.start();
        assert_eq!(start_res.is_err(), true);

        let mut actual: usize;
        actual = game.end_phase().unwrap();
        assert_eq!(actual, 1);
        actual = game.end_phase().unwrap();
        assert_eq!(actual, 2);
        actual = game.end_phase().unwrap();
        assert_eq!(actual, 3);

        let end_res = game.end_phase();
        assert_eq!(end_res.is_err(), true);
    }
    
    #[test]
    fn full_game_works() {
        let mut game = Game::new();

        let part_1 = GamePhase::new(String::from("Primera parte"), 8);
        let part_intermedio = GamePhase::new(String::from("Descanso"), 0);
        let part_2 = GamePhase::new(String::from("Segunda parte"), 8);

        game.set_phases(vec![part_1, part_intermedio, part_2]);
        assert_eq!(game.phases_count(), 3);

        let start_res = game.start();
        assert_eq!(start_res.is_err(), true);

        let mut actual: usize;
        actual = game.end_phase().unwrap();
        assert_eq!(actual, 1);
        actual = game.end_phase().unwrap();
        assert_eq!(actual, 2);
        actual = game.end_phase().unwrap();
        assert_eq!(actual, 3);

    }
}
