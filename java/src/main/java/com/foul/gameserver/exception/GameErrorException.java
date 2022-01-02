package com.foul.gameserver.exception;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

/**
 * GameErrorException
 */
public class GameErrorException extends Exception {

    Logger logger = LoggerFactory.getLogger(GameErrorException.class);

    public GameErrorException(String errorMessage) {
        super(errorMessage);
    }

    public GameErrorException(GameError gameError) {
        super(gameError.toString());
        logger.error(this.getMessage(), this);        
    }


}

