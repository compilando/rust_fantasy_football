package com.foul.gameserver.games.FfGame;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import lombok.ToString;
import com.foul.gameserver.domain.base.Piece;
import com.foul.gameserver.domain.base.Race;
import com.foul.gameserver.domain.base.Skill;
import com.foul.gameserver.domain.base.Team;
import com.foul.gameserver.domain.geometry.Square;

@Getter
@RequiredArgsConstructor
@ToString
public final class FfPiece extends Piece {

	Logger logger = LoggerFactory.getLogger(FfPiece.class);

	Race race;
	String title;
	int cost;
	int ma;
	int st;
	int ag;
	int av;
	int number;
	Skill skills;
	Square position;
	Team team;
   
}
