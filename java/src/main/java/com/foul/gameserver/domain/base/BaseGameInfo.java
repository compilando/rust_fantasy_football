package com.foul.gameserver.domain.base;

import com.foul.gameserver.exception.GameErrorException;

public interface BaseGameInfo {

    String getName() throws GameErrorException;

    BaseBoard getBoard() throws GameErrorException;
    
}
