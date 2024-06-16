-- This file should undo anything in `up.sql`
CREATE TABLE `player`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`name` VARCHAR(20) NOT NULL,
	`elo` INTEGER NOT NULL
);

DROP TABLE IF EXISTS `players`;
