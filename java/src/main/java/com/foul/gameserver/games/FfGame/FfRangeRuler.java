package com.foul.gameserver.games.FfGame;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import lombok.EqualsAndHashCode;
import lombok.Getter;
import lombok.ToString;

@Getter
@EqualsAndHashCode
@ToString
public final class FfRangeRuler {

    Logger logger = LoggerFactory.getLogger(FfRangeRuler.class);

    int quickPass;
    int shortPass;
    int longPass;
    int longBomb;
   
    FfRangeRuler() {
        this.quickPass = 3;
        this.shortPass = 7;
        this.longPass = 10;
        this.longBomb = 13;
    }

    FfPassRangeEnum getPassRange(int distance){
		
        FfRangeRuler rr = new FfRangeRuler();

        if( distance > rr.longBomb ) {
			return FfPassRangeEnum.OutOfRange;
		} else if( distance > rr.longPass ) {
			return FfPassRangeEnum.LongBomb;
		} else if( distance > rr.shortPass ) {
			return FfPassRangeEnum.LongPass;
		} else if( distance > rr.quickPass ) {
			return FfPassRangeEnum.ShortPass;
		} else {
			return FfPassRangeEnum.QuickPass;
		}		
	}
}
