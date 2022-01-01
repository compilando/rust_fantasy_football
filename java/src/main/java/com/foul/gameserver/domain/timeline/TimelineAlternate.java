package com.foul.gameserver.domain.timeline;


import com.foul.gameserver.domain.base.GamePhase;
import lombok.EqualsAndHashCode;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import lombok.ToString;

@Getter
@EqualsAndHashCode
@RequiredArgsConstructor
@ToString
public final class TimelineAlternate implements TimelineBase {
    int x;
    int y;

    @Override
    public GamePhase evolveGame() {
        return null;
    //    let current_phase = game.current_phase();
    //    let current_player = game.current_player();
    //    let result = game.end_phase();
    }
}
