use crate::models::{basegame::{BaseGame, BaseGameLifeCycle}, gameerror::GameError, gamephase::GamePhase, team::Team};

pub struct FfGame {
    game: BaseGame,
}

impl FfGame {
	pub fn new() -> FfGame {
        let game = BaseGame {
            name: String::from("Fantasy Football"),
            phases: Vec::new(),
            teams: Vec::new(),
            current_phase: 0
        };
        FfGame {
            game            
        }
    }

    pub fn set_phases(&mut self, phases: Vec<GamePhase>) {
        self.game.set_phases(phases)
    }

    pub fn set_teams(&mut self, teams: Vec<Team>) {
        self.game.set_teams(teams)
    }
  
}

impl BaseGameLifeCycle for FfGame {

    fn name (self: &'_ Self) -> Option<&'_ String> {
        self.game.name()
    }

    fn start(&mut self) -> Result<(), GameError> {
        self.game.start()
    }

    fn end_phase(&mut self) -> Result<usize, GameError> {
        self.game.end_phase()
    }
    
    fn phase_count(&mut self) -> usize {
        self.game.phase_count()
    }

    fn current_phase(&mut self) -> Result<usize, GameError> {
        Ok(self.game.current_phase)
    }

    fn end(&mut self) -> Result<(), GameError> {
        self.game.end()
    }
   
}

#[cfg(test)]
mod tests {
    use crate::models::{basegame::BaseGameLifeCycle, games::ff_game::ff_game::FfGame, team::Team};
    pub(crate) use crate::models::{gamephase::GamePhase};

    #[test]
    fn new_game_works() {
        let mut game = FfGame::new();

        // GamePhase init
        let part_1 = GamePhase::new(String::from("Primera parte"), 8);
        let part_intermedio = GamePhase::new(String::from("Descanso"), 0);
        let part_2 = GamePhase::new(String::from("Segunda parte"), 8);
        game.set_phases(vec![part_1, part_intermedio, part_2]);
        assert_eq!(game.phase_count(), 3);

        // Teams init
        let team_a = Team::new(String::from("Orcos del inframundo"));
        let team_b = Team::new(String::from("Enfermizas"));
        game.set_teams(vec![team_a, team_b]);

        // Start the game
        let start_res = game.start();
        assert_eq!(start_res.is_err(), false);

        //let expected = Ok(3);
        //assert_eq!(game.current_phase(), expected);
    }

    #[test]
    fn end_phase_past_works() {
        let mut game = FfGame::new();

        // GamePhase init
        let part_1 = GamePhase::new(String::from("Primera parte"), 8);
        let part_intermedio = GamePhase::new(String::from("Descanso"), 0);
        let part_2 = GamePhase::new(String::from("Segunda parte"), 8);
        game.set_phases(vec![part_1, part_intermedio, part_2]);
        assert_eq!(game.phase_count(), 3);

        // Teams init
        let team_a = Team::new(String::from("Orcos del inframundo"));
        let team_b = Team::new(String::from("Enfermizas"));
        game.set_teams(vec![team_a, team_b]);

        // Start the game
        let start_res = game.start();
        assert_eq!(start_res.is_err(), false);

        // Play the game
        let mut actual: usize;
        actual = game.end_phase().unwrap();
        assert_eq!(actual, 2);
        actual = game.end_phase().unwrap();
        assert_eq!(actual, 3);
        actual = game.end_phase().unwrap();
        assert_eq!(actual, 3);

        // Try to end inexistent phase...
        let end_res = game.end_phase();
        assert_eq!(end_res.is_err(), true);
    }
    
    #[test]
    fn full_game_works() {
        let mut game = FfGame::new();

        let part_1 = GamePhase::new(String::from("Primera parte"), 8);
        let part_intermedio = GamePhase::new(String::from("Descanso"), 0);
        let part_2 = GamePhase::new(String::from("Segunda parte"), 8);

        game.set_phases(vec![part_1, part_intermedio, part_2]);
        assert_eq!(game.phase_count(), 3);

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