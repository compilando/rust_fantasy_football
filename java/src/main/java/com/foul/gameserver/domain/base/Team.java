package com.foul.gameserver.domain.base;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import java.util.List;
import lombok.EqualsAndHashCode;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import lombok.ToString;

@Getter
@EqualsAndHashCode
@RequiredArgsConstructor
@ToString
public final class Team {

	Logger logger = LoggerFactory.getLogger(Team.class);

    List<Piece> players;
	int rerolls;
	int fanFactor;
	int assistantCoaches;
	int cheerleaders;
	TeamStatus teamStatus;
	String name;

	
	public Team(String name) {
		this.name = name;
	}

}
