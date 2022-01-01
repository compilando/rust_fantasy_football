#[derive(Debug)]
pub enum GameError {
    PhasesNotDefined,
    TeamsNotDefined,
    GameNotStarted,
    GameAlreadyFinished,
}