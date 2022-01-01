package com.foul.gameserver.domain.base;

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
public final class TeamStatus {

	Logger logger = LoggerFactory.getLogger(TeamStatus.class);

	public void reset() {
	}
}

