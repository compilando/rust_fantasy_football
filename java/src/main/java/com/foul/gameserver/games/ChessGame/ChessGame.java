package com.foul.gameserver.games.ChessGame;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import lombok.Getter;
import lombok.ToString;
import com.foul.gameserver.domain.base.BaseGame;

@Getter
@ToString
public final class ChessGame extends BaseGame {

    Logger logger = LoggerFactory.getLogger(ChessGame.class);

    public ChessGame() {
        super();
        this.gameBoard = new ChessBoard();
        this.name = "Chess";
    }
   
}
