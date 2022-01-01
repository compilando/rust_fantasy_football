package com.foul.gameserver;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@SpringBootApplication
public class GameserverApplication {

	Logger logger = LoggerFactory.getLogger(GameserverApplication.class);

	public static void main(String[] args) {
		SpringApplication.run(GameserverApplication.class, args);
	}

}
