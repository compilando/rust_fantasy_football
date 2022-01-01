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

    public ChessBoard() {
        this.board = new int[8][8];
    }

    @Override
    public String draw() {
        String result = "";
        result += "---------------------------------" + "\r\n";
        for (int i = 0; i < 8; i++) {
            for (int j = 0; j < 8; j++) {
                result += "| " + board[i][j] + " ";
            }            
            result += "| \r\n";
        }
        result += "---------------------------------" + "\r\n";
        return result;
    }
}
