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

  PRIMARY KEY (id, apikey),
  UNIQUE KEY (symbol, apikey),

  FOREIGN KEY (apikey) REFERENCES apikeys (apikey)
);

CREATE TABLE accounts (
  id INT UNSIGNED NOT NULL AUTO_INCREMENT,
  apikey VARCHAR(45) NOT NULL,
  currency_id INT UNSIGNED NOT NULL,
  title VARCHAR(45) NOT NULL,

  PRIMARY KEY (id, apikey),
  FOREIGN KEY (apikey) REFERENCES apikeys (apikey),
  FOREIGN KEY (currency_id, apikey) REFERENCES currencies (id, apikey)
);

CREATE TABLE transactions (
  id INT UNSIGNED NOT NULL AUTO_INCREMENT,
  apikey VARCHAR(45) NOT NULL,
  notes TEXT NOT NULL,
  time VARCHAR(45) NOT NULL,

  PRIMARY KEY (id, apikey),
  FOREIGN KEY (apikey) REFERENCES apikeys (apikey)
);

CREATE TABLE distributions (
  id INT UNSIGNED NOT NULL AUTO_INCREMENT,
  account_id INT UNSIGNED NOT NULL,
  amount BIGINT NOT NULL,
  amount_exp TINYINT NOT NULL,
  apikey VARCHAR(45) NOT NULL,
  transaction_id INT UNSIGNED NOT NULL,

  PRIMARY KEY (id, apikey),
  FOREIGN KEY (account_id, apikey) REFERENCES accounts (id, apikey),
  FOREIGN KEY (transaction_id, apikey) REFERENCES transactions (id, apikey)

);

CREATE TABLE categories (
  id INT UNSIGNED NOT NULL AUTO_INCREMENT,
  apikey VARCHAR(45) NOT NULL,
  symbol VARCHAR(45) NOT NULL,
  title VARCHAR(45) NOT NULL,

  PRIMARY KEY (id, apikey),
  UNIQUE KEY (symbol, apikey),

  FOREIGN KEY (apikey) REFERENCES apikeys (apikey)
);

CREATE TABLE accounts_categories (
  id INT UNSIGNED NOT NULL AUTO_INCREMENT,
  apikey VARCHAR(45) NOT NULL,
  account_id INT UNSIGNED NOT NULL,
  category_id INT UNSIGNED NOT NULL,

  PRIMARY KEY (id, apikey),
  UNIQUE KEY (account_id, category_id),

  FOREIGN KEY (apikey) REFERENCES apikeys (apikey),
  FOREIGN KEY (account_id, apikey) REFERENCES accounts (id, apikey),
  FOREIGN KEY (category_id, apikey) REFERENCES categories (id, apikey)

);

CREATE TABLE transactions_categories (
  id INT UNSIGNED NOT NULL AUTO_INCREMENT,
  apikey VARCHAR(45) NOT NULL,
  transaction_id INT UNSIGNED NOT NULL,
  category_id INT UNSIGNED NOT NULL,

  PRIMARY KEY (id, apikey),
  UNIQUE KEY (transaction_id, category_id),

  FOREIGN KEY (apikey) REFERENCES apikeys (apikey),
  FOREIGN KEY (transaction_id, apikey) REFERENCES transactions (id, apikey),
  FOREIGN KEY (category_id, apikey) REFERENCES categories (id, apikey)

);