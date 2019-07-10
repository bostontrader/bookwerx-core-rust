DROP TABLE IF EXISTS `accounts`;
CREATE TABLE `accounts` (
  `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
  `amount` decimal(18,8) unsigned zerofill NOT NULL,
  `account_name` text NOT NULL,
PRIMARY KEY (`id`)
);

DROP TABLE IF EXISTS `currencies`;
CREATE TABLE `currencies` (
  `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
  `symbol` varchar(45) NOT NULL,
  `title` varchar(45) NOT NULL,
  PRIMARY KEY (`id`)
);



