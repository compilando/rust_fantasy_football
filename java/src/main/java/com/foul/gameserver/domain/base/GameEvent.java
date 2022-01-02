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
public final class GameEvent {

    Logger logger = LoggerFactory.getLogger(GameEvent.class);

    @Getter
    @Setter
    private String property;

    @Getter
    @Setter
    private String value;

}
