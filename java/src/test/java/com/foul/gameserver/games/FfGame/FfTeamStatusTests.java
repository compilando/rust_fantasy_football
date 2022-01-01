package com.foul.gameserver.games.FfGame;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import static org.junit.jupiter.api.Assertions.assertEquals;
import org.junit.jupiter.api.Test;
import org.springframework.boot.test.context.SpringBootTest;

@SpringBootTest
class FfTeamStatusTests {

	Logger logger = LoggerFactory.getLogger(FfTeamStatusTests.class);

	@Test
	void resetTest() {
		var teamStatus = new FfTeamStatus();

		teamStatus.rerolledThisTurn = true;
		teamStatus.hasBlitzed = true;
		teamStatus.hasFouled = true;
		teamStatus.hasPassed = true;
		teamStatus.hasHandedOf = true;
        teamStatus.reset();

        assertEquals(teamStatus.rerolledThisTurn, false);
		assertEquals(teamStatus.hasBlitzed, false);
		assertEquals(teamStatus.hasFouled, false);
		assertEquals(teamStatus.hasPassed, false);
		assertEquals(teamStatus.hasHandedOf, false);
		
	}

}
