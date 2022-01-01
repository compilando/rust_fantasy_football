package com.foul.gameserver.games.FfGame;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
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
public final class FfTeamStatus {

	Logger logger = LoggerFactory.getLogger(FfTeamStatus.class);

	int score;
	int rerolls;
	boolean rerolledThisTurn;
	int fans;
	int fame;
	int babes;
	boolean hasBlitzed;
	boolean hasFouled;
	boolean hasPassed;
	boolean hasHandedOf;

	public void reset() {
		this.rerolledThisTurn = false;
		this.hasBlitzed = false;
		this.hasFouled = false;
		this.hasPassed = false;
		this.hasHandedOf = false;
	}

}

