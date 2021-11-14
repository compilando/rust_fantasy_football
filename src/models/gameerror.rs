#[derive(Debug)]
pub enum GameError {
    PhasesNotDefined,
    TeamsNotDefined,
    GameAlreadyFinished,
}