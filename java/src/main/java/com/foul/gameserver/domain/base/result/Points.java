package com.foul.gameserver.domain.base.result;

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
public final class Points implements PointAccountable {

    int quantity = 0;

    @Override
    public Points total() {
        return this;
    }

}
