use super::{gameerror::GameError, team::Team};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct BaseGame {
    pub name: String,
    pub teams: Vec<Team>,
    pub phases: Vec<super::gamephase::GamePhase>,
    pub current_phase: usize
}

pub trait BaseGameLifeCycle {
    fn name (self: &'_ Self) -> Option<&'_ String>;
    fn start_game(&mut self) -> Result<(), GameError>;
    fn start_phase(&mut self) -> Result<usize, GameError>;
    fn end_phase(&mut self) -> Result<usize, GameError>;
    fn phase_count(&mut self) -> usize;
    fn current_phase(&mut self) -> Result<usize, GameError>;    
    fn end_game(&mut self) -> Result<(), GameError>;
}

impl BaseGame {
	pub fn new() -> BaseGame {
        BaseGame {
            name: String::from("None"),
            phases: Vec::new(),
            teams:  Vec::new(),
            current_phase: 0
        }
    }

    pub fn set_phases(&mut self, phases: Vec<super::gamephase::GamePhase>) {
        for item in phases {
            let _ = &self.phases.push(item);
        }
    }

    pub fn set_teams(&mut self, teams: Vec<super::team::Team>) {
        for item in teams {
            let _ = &self.teams.push(item);
        }
    }

    pub fn phases_count(&mut self) -> usize {
        self.phases.len()
    }

}

impl BaseGameLifeCycle for BaseGame {

    fn name (self: &'_ Self) -> Option<&'_ String> {
        Some(&self.name)
    }

    fn start_game(&mut self) -> Result<(), GameError> {
        
        let phases_count = self.phase_count();
        if phases_count == 0 {
            return Err(GameError::PhasesNotDefined);
        }

        if self.teams.len() < 2 {
            return Err(GameError::TeamsNotDefined);
        }
        self.current_phase = 1;
        Ok(())
    }

    fn phase_count(&mut self) -> usize {
        self.phases.len()
    }

    fn current_phase(&mut self) -> Result<usize, GameError> {
        Ok(self.current_phase)
    }

    fn start_phase(&mut self) -> Result<usize, GameError> {
        Ok(self.current_phase) // TODO
    }

    fn end_phase(&mut self) -> Result<usize, GameError> {
        let phase_len = self.phases.len();

        if self.current_phase==0 {
            return Err(GameError::GameNotStarted);
        }

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

    fn end_game(&mut self) -> Result<(), GameError> {
        Ok(())
    }


   
}
