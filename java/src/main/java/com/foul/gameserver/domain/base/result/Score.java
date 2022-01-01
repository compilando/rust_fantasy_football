package com.foul.gameserver.domain.base.result;

import com.foul.gameserver.domain.base.Team;
import lombok.EqualsAndHashCode;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import lombok.Setter;
import lombok.ToString;



@Getter
@EqualsAndHashCode
@RequiredArgsConstructor
@ToString
public final class Score {

    @Getter
    @Setter
    private Team team;

    @Getter
    @Setter
    private Points points;
}
