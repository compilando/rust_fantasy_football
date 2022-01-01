package com.foul.gameserver.games.FfGame;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import com.foul.gameserver.domain.geometry.Square;
import lombok.EqualsAndHashCode;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import lombok.ToString;

@Getter
@EqualsAndHashCode
@RequiredArgsConstructor
@ToString
public final class FfBall {

    Logger logger = LoggerFactory.getLogger(FfBall.class);

    Square square;
    boolean inGame;
    boolean underControl;
    boolean onGround;

}
