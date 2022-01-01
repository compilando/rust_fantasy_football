package com.foul.gameserver.domain.geometry;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import com.foul.gameserver.domain.base.Piece;
import lombok.EqualsAndHashCode;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import lombok.ToString;

@Getter
@EqualsAndHashCode
@RequiredArgsConstructor
@ToString
public final class Square {
    
    Logger logger = LoggerFactory.getLogger(Square.class);

    int x;
    int y;
    Piece piece;
}
