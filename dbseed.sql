DROP TABLE IF EXISTS currencies;
CREATE TABLE currencies (
  id int(11) unsigned NOT NULL AUTO_INCREMENT,
  symbol varchar(45) NOT NULL,
  title varchar(45) NOT NULL,
  UNIQUE KEY (symbol),
  PRIMARY KEY (id)
);

DROP TABLE IF EXISTS accounts;
CREATE TABLE accounts (
  id int(11) unsigned NOT NULL AUTO_INCREMENT,
  currency_id int(11) unsigned NOT NULL,
  title varchar(45) NOT NULL,
  PRIMARY KEY (id)
);

/* For some reason I cannot include this foreign key constraint in the initial create statement.  So add it here instead. */
ALTER TABLE accounts
  ADD FOREIGN KEY (currency_id) REFERENCES currencies (id);
