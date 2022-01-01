package com.foul.gameserver.domain.base;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;
import com.foul.gameserver.domain.base.result.GameResult;
import com.foul.gameserver.domain.base.result.PhaseResult;
import com.foul.gameserver.exception.GameError;
import com.foul.gameserver.exception.GameErrorException;
import lombok.EqualsAndHashCode;
import lombok.Getter;
import lombok.Setter;
import lombok.ToString;

@Getter
@EqualsAndHashCode
@ToString
public class BaseGame implements BaseGameLifeCycle, BaseGameInfo {

    Logger logger = LoggerFactory.getLogger(BaseBoard.class);
    
    @Setter
    protected String name;

    @Setter
    protected List<Team> teams;

    @Setter
    protected List<GamePhase> phases;

    @Setter
    protected GamePhase currentPhase;

    @Setter
    protected int currentPhaseNumber;

    @Setter
    protected Player currentPlayer;

    @Setter
    protected BaseBoard gameBoard;

    public BaseGame() {
        this.currentPhaseNumber = 0;
    }

    protected int getMinTeamsSize() {
        return 1;
    }

    @Override
    public GameResult startGame() throws GameErrorException {
        logger.debug("BaseGame::startGame");

        var phaseCount = this.phaseCount();
        if (phaseCount == 0) {
            throw new GameErrorException(GameError.PhasesNotDefined);
        }

        if (this.teams.size() < this.getMinTeamsSize()) {
            throw new GameErrorException(GameError.TeamsNotDefined);
        }
        this.currentPhaseNumber = 1;
        this.currentPhase = this.getNextPhase(null);
        // TODO Phase real
        return new GameResult();
    }

    @Override
    public PhaseResult startPhase() throws GameErrorException {
        return new PhaseResult(); // TODO
    }

    @Override
    public PhaseResult endPhase() throws GameErrorException {
        if (this.currentPhase() == null) {
            throw new GameErrorException(GameError.GameNotStarted);
        }

        var phaseLen = this.phaseCount();
        if (this.currentPhaseNumber() > phaseLen) {
            throw new GameErrorException(GameError.GameAlreadyFinished);
        } else if (this.currentPhaseNumber() == phaseLen) {
            this.currentPhaseNumber += 1;
            return new PhaseResult(); // TODO
        } else {
            this.currentPhaseNumber += 1;
            return new PhaseResult(); // TODO
        }
    }

    public BaseBoard getBoard() throws GameErrorException {
        if (this.gameBoard == null) {
            throw new GameErrorException("No board defined");
        }
        return this.gameBoard;
    }

    @Override
    public GameResult endGame() throws GameErrorException {
        return null;
    }

    @Override
    public int phaseCount() {
        return this.phases.size();
    }

    @Override
    public GamePhase currentPhase() {
        return this.currentPhase;
    }

    @Override
    public int currentPhaseNumber() {
        return this.currentPhaseNumber;
    }

    @Override
    public Player currentPlayer() {
        return this.currentPlayer;
    }

    private GamePhase getNextPhase(GamePhase phase) {
        if (phase == null) {
            return this.phases.get(0);
        }

        return this.phases.stream().filter(item -> phase.getUid().equals(item.getUid())).findFirst()
                .orElse(null);
    }

}
