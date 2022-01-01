package com.foul.gameserver.games.ChessGame;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import static org.junit.jupiter.api.Assertions.fail;
import com.foul.gameserver.exception.GameErrorException;
import org.junit.jupiter.api.Test;
import org.springframework.boot.test.context.SpringBootTest;

@SpringBootTest
class ChessGameTests {

	Logger logger = LoggerFactory.getLogger(ChessGameTests.class);

	@Test
	void chessGameDrawTest() {
		ChessGame game = new ChessGame();
		try {
			var board = game.getBoard().draw();
			System.out.println(board);
		} catch (GameErrorException e) {			
			fail("Game Error: " + e.getMessage());
		}		
	}

}
