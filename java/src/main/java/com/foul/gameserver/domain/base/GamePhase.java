package com.foul.gameserver.domain.base;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import lombok.EqualsAndHashCode;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import lombok.Setter;
import lombok.ToString;

@Getter
@EqualsAndHashCode
@RequiredArgsConstructor
@ToString
public final class GamePhase {

    Logger logger = LoggerFactory.getLogger(GamePhase.class);

    @Getter
    @Setter
    private String uid;

    @Getter
    @Setter
    private String name;
    
    @Getter
    @Setter
    private int turns;
    
    @Getter
    @Setter
    private boolean done;

    @Getter
    @Setter
    private Player player;

    public GamePhase(String uid, String name, int turns) {
        this.uid = uid;
        this.name = name;
        this.turns = turns;
    }
}
