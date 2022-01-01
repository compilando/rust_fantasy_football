package com.foul.gameserver.domain.base;

import com.foul.gameserver.domain.base.result.GameResult;
import com.foul.gameserver.domain.base.result.PhaseResult;
import com.foul.gameserver.exception.GameErrorException;

public interface BaseGameLifeCycle {

    GameResult startGame() throws GameErrorException;

    PhaseResult startPhase() throws GameErrorException;

    PhaseResult endPhase() throws GameErrorException;

    GameResult endGame() throws GameErrorException;

    int phaseCount();

    GamePhase currentPhase();
    
    int currentPhaseNumber();

    Player currentPlayer();
    
}
