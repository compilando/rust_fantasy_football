package com.foul.gameserver.games.FfGame;

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
public final class FfBlock {
	
	Logger logger = LoggerFactory.getLogger(FfBlock.class);

	Piece attacker;
	Piece defender;
   
}
