package com.foul.gameserver.games.ChessGame;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import com.foul.gameserver.domain.base.BaseBoard;
import lombok.Getter;
import lombok.ToString;

@Getter
@ToString
public final class ChessBoard extends BaseBoard {

    Logger logger = LoggerFactory.getLogger(ChessBoard.class);

    int[][] board;

    int BOARD_WIDTH = 8;
    int BOARD_HEIGHT = 8;

    public ChessBoard() {
        this.board = new int[BOARD_WIDTH][BOARD_HEIGHT];
    }

    @Override
    public String draw() {
        String result = "";
        result += "---------------------------------" + "\r\n";
        for (int i = 0; i < BOARD_WIDTH; i++) {
            for (int j = 0; j < BOARD_HEIGHT; j++) {
                result += "| " + board[i][j] + " ";
            }            
            result += "| \r\n";
        }
        result += "---------------------------------" + "\r\n";
        return result;
    }
}
