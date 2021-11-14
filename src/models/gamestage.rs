use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum GameStage {
	StartUp, 
	CoinToss,
	PickCoinTossEffect,
	KickingSetup,
	ReceivingSetup,
	KickPlacement,
	PlaceBallOnPlayer,
	KickOff,
	QuickSnap,
	Blitz,
	PerfectDefense,
	HighKick,
	HomeTurn,
	AwayTurn,
	GameEnded
	
}

