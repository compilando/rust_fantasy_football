package com.foul.gameserver.games.ChessGame;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import com.foul.gameserver.domain.base.ConsoleWritableSymbol;
import com.foul.gameserver.domain.base.ValidMoveable;
import lombok.EqualsAndHashCode;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import lombok.ToString;

@Getter
@EqualsAndHashCode
@RequiredArgsConstructor
@ToString
public final class ChessRook implements ConsoleWritableSymbol, ValidMoveable {

    Logger logger = LoggerFactory.getLogger(ChessRook.class);

    @Override
    public String symbol() {
        return "R";
    }

    @Override
    public void validMoves() {
                
    }
  
}
