package com.foul.gameserver.games.FfGame;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import java.util.List;
import com.foul.gameserver.domain.base.Piece;
import com.foul.gameserver.domain.base.Team;
import lombok.EqualsAndHashCode;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import lombok.ToString;

@Getter
@EqualsAndHashCode
@RequiredArgsConstructor
@ToString
public final class FfDugout {
	Logger logger = LoggerFactory.getLogger(FfDugout.class);
	
	Team team;
	List<Piece> reserves;
	List<Piece> knocked_out;
	List<Piece> dead_and_injured;
   
}
