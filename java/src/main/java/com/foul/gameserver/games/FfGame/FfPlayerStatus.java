package com.foul.gameserver.games.FfGame;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import lombok.EqualsAndHashCode;
import lombok.Getter;
import lombok.ToString;

@Getter
@EqualsAndHashCode
@ToString
public final class FfPlayerStatus {

    Logger logger = LoggerFactory.getLogger(FfPlayerStatus.class);

    FfStanding standing;
	FfPlayerTurn turn;
	int movementUsed;
	boolean movedToBlock;

    FfPlayerStatus() {
        this.standing = FfStanding.Up;
        this.turn = FfPlayerTurn.Unused;
        this.movementUsed = 0;
        this.movedToBlock = false;
    }
   
}
