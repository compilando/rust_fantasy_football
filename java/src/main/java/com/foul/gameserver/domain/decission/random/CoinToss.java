package com.foul.gameserver.domain.decission.random;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import lombok.EqualsAndHashCode;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import lombok.ToString;

@Getter
@EqualsAndHashCode
@RequiredArgsConstructor
@ToString
public final class CoinToss {
	
	Logger logger = LoggerFactory.getLogger(CoinToss.class);

	boolean tossed;
	boolean awayPickedHeads;
	boolean resultHeads;
	boolean homeReceives;
  
}