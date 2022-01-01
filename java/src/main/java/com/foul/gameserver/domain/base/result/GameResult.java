package com.foul.gameserver.domain.base.result;

import java.util.List;
import com.foul.gameserver.domain.base.GameAttr;
import com.foul.gameserver.domain.base.GameEvent;
import com.foul.gameserver.domain.base.Team;
import lombok.EqualsAndHashCode;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import lombok.Setter;
import lombok.ToString;

@Getter
@Setter
@EqualsAndHashCode
@RequiredArgsConstructor
@ToString
public final class GameResult {

    private Team winner;

    private List<Score> scores;
    
    private List<GameAttr> attrs;

    private List<GameEvent> events;
}
