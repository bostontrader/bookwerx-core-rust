CREATE TABLE apikeys (
  id int(11) unsigned NOT NULL AUTO_INCREMENT,
  apikey varchar(45) NOT NULL,
  PRIMARY KEY (id),
  UNIQUE KEY (apikey)
);

CREATE TABLE currencies (
  id int(11) unsigned NOT NULL AUTO_INCREMENT,
  symbol varchar(45) NOT NULL,
  title varchar(45) NOT NULL,
  apikey varchar(45) NOT NULL,

  PRIMARY KEY (id),
  UNIQUE KEY (symbol),
  FOREIGN KEY (apikey) REFERENCES apikeys (apikey)
);

CREATE TABLE accounts (
  id int(11) unsigned NOT NULL AUTO_INCREMENT,
  currency_id int(11) unsigned NOT NULL,
  title varchar(45) NOT NULL,
  apikey varchar(45) NOT NULL,

  PRIMARY KEY (id),
  FOREIGN KEY (apikey) REFERENCES apikeys (apikey),
  FOREIGN KEY (currency_id) REFERENCES currencies (id)
);

