package com.foul.gameserver.games.FfGame;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.fail;
import java.util.ArrayList;
import java.util.List;
import com.foul.gameserver.domain.base.Team;
import com.foul.gameserver.exception.GameErrorException;
import org.junit.jupiter.api.Test;
import org.springframework.boot.test.context.SpringBootTest;

@SpringBootTest
class FfGameTests {

	Logger logger = LoggerFactory.getLogger(FfGameTests.class);

	@Test
	void chessGameDrawTest() {
		FfGame game = new FfGame();

		try {
			var board = game.getBoard().draw();
			System.out.println(board);
		} catch (GameErrorException e) {
			fail("Game Error: " + e.getMessage());
		}
	}

	@Test
	void newGameWorksTest() {
		FfGame game = new FfGame();
		assertEquals(game.phaseCount(), 3);

		// Teams init
		var teamA = new Team("Orcos del inframundo");
		var teamB = new Team("Enfermizas");
		List<Team> teams = new ArrayList<Team>();
		teams.add(teamA);
		teams.add(teamB);

		game.setTeams(teams);

		// Start the game
		try {
			game.startGame();
		} catch (GameErrorException e) {
			e.printStackTrace();
			fail("Game Start Error: " + e.getMessage());
		}

		assertEquals(game.currentPhase().getUid(), "FF_GAME_FIRST_PART");

	}

	@Test
	void endPhasePastWorksTest() {
		var game = new FfGame();
		assertEquals(game.phaseCount(), 3);

		// Teams init
		var teamA = new Team("Orcos del inframundo");
		var teamB = new Team("Enfermizas");
		List<Team> teams = new ArrayList<Team>();
		teams.add(teamA);
		teams.add(teamB);

		game.setTeams(teams);

		// Start the game
		try {
			game.startGame();
		} catch (GameErrorException e) {
			e.printStackTrace();
			fail("Game Start Error");
		}

		// Play the game
		try {
			var actual = game.endPhase();
			// TODO assertEquals(actual, 2);
			actual = game.endPhase();
			// TODO assertEquals(actual, 3);
			actual = game.endPhase();
			// TODO assertEquals(actual, 3);
		} catch (GameErrorException e) {
			e.printStackTrace();
			fail("End Phase Error: " + e.getMessage());
		}

		// Try to end inexistent phase...
		try {
			game.endPhase();
		} catch (GameErrorException e) {
			// It's ok to fail
		}

		try {
			game.endGame();
		} catch (GameErrorException e) {
			e.printStackTrace();
			fail("End Game Error: " + e.getMessage());
		}
	}

	@Test
	void passRangeTest() {
		FfRangeRuler rr = new FfRangeRuler();

		assertEquals(rr.getPassRange(2), FfPassRangeEnum.QuickPass);
	}

}
