/* The order of definition is important lest we trip over referential and foreign key issues */
CREATE TABLE apikeys (
  id INT UNSIGNED NOT NULL AUTO_INCREMENT,
  apikey VARCHAR(45) NOT NULL,
  PRIMARY KEY (id),
  UNIQUE KEY (apikey)
);

CREATE TABLE currencies (
  id INT UNSIGNED NOT NULL AUTO_INCREMENT,
  apikey VARCHAR(45) NOT NULL,
  symbol VARCHAR(45) NOT NULL,
  title VARCHAR(45) NOT NULL,

  PRIMARY KEY (id),
  UNIQUE KEY (symbol),
  FOREIGN KEY (apikey) REFERENCES apikeys (apikey)
);

CREATE TABLE accounts (
  id INT UNSIGNED NOT NULL AUTO_INCREMENT,
  apikey VARCHAR(45) NOT NULL,
  currency_id INT UNSIGNED NOT NULL,
  title VARCHAR(45) NOT NULL,

  PRIMARY KEY (id),
  FOREIGN KEY (apikey) REFERENCES apikeys (apikey),
  FOREIGN KEY (currency_id) REFERENCES currencies (id)
);

CREATE TABLE transactions (
  id INT UNSIGNED NOT NULL AUTO_INCREMENT,
  apikey VARCHAR(45) NOT NULL,
  notes TEXT NOT NULL,

  PRIMARY KEY (id),
  FOREIGN KEY (apikey) REFERENCES apikeys (apikey)
);

/* A distribution doesn't need its own apikey because its related to accounts and transactions that do have them. */
CREATE TABLE distributions (
  id INT UNSIGNED NOT NULL AUTO_INCREMENT,
  account_id INT UNSIGNED NOT NULL,
  amount BIGINT NOT NULL,
  amount_exp TINYINT NOT NULL,
  transaction_id INT UNSIGNED NOT NULL,

  PRIMARY KEY (id),
  FOREIGN KEY (account_id) REFERENCES accounts (id),
  FOREIGN KEY (transaction_id) REFERENCES transactions (id)

);
