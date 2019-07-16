CREATE TABLE apikeys (
  id int(11) unsigned NOT NULL AUTO_INCREMENT,
  apikey varchar(45) NOT NULL,
  UNIQUE KEY (apikey),
  PRIMARY KEY (id)
);

CREATE TABLE currencies (
  id int(11) unsigned NOT NULL AUTO_INCREMENT,
  symbol varchar(45) NOT NULL,
  title varchar(45) NOT NULL,
  UNIQUE KEY (symbol),
  PRIMARY KEY (id)
);

CREATE TABLE accounts (
  id int(11) unsigned NOT NULL AUTO_INCREMENT,
  currency_id int(11) unsigned NOT NULL,
  title varchar(45) NOT NULL,
  PRIMARY KEY (id)
);

/* For some reason I cannot include this foreign key constraint in the initial create statement.  So add it here instead. */
ALTER TABLE accounts
  ADD FOREIGN KEY (currency_id) REFERENCES currencies (id);
