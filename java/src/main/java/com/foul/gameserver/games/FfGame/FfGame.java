package com.foul.gameserver.games.FfGame;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import java.util.ArrayList;
import java.util.List;
import com.foul.gameserver.domain.base.BaseGame;
import com.foul.gameserver.domain.base.GamePhase;
import lombok.Getter;
import lombok.ToString;

@Getter
@ToString
public final class FfGame extends BaseGame {

    Logger logger = LoggerFactory.getLogger(FfGame.class);

    FfGame() {
        super();

        this.gameBoard = new FfBoard();

        this.name = "Fantasy Football";

        var part1 = new GamePhase("FF_GAME_FIRST_PART", "Primera parte", 8);
		var partIntermedio = new GamePhase("FF_GAME_MIDDLE_GAME", "Descanso", 0);
		var part2 = new GamePhase("FF_GAME_SECOND_PART", "Segunda parte", 8);

		List<GamePhase> phases = new ArrayList<GamePhase>();
		phases.add(part1);
		phases.add(partIntermedio);
		phases.add(part2);
		this.setPhases(phases);
    }

    @Override
    protected int getMinTeamsSize() {
        return 2;
    }
}
