# Introduction

[![Build Status](https://travis-ci.org/bostontrader/bookwerx-core-rust.png?branch=master)](https://travis-ci.org/bostontrader/bookwerx-core-rust)
[![MIT license](http://img.shields.io/badge/license-MIT-brightgreen.svg)](http://opensource.org/licenses/MIT)

The purpose of ***bookwerx-core-rust*** is to provide an API that supports multi-currency
 bookkeeping, using the double-entry bookkeeping model, slightly adapted to squeeze 
 in multiple currencies.  It uses [rust](https://www.rust-lang.org), [rocket](https://rocket.rs), and [mysql](https://www.mysql.com).

Any application that deals with "money" (fiat, precious metals, cryptocoins) will
quickly encounter the need for bookkeeping.  Rolling your own methods is, as usual,
 easier said than done, so perhaps you can save yourself some grief and enjoy ***bookwerx-core-rust*** instead.

With this API, the user can:

* Perform ordinary CRUD operations on the various bookkeeping objects,
such as accounts, currencies, and transactions.

* Perform consistency checks.


## Getting Started

### Prerequisites

* You will need rust.

* You will need git.

* You will need mysql.


The care and feeding of these items are beyond the scope of these instructions.

### But assuming they are correctly installed...

```bash
git clone https://github.com/bostontrader/bookwerx-core-rust.git
cd bookwerx-core-rust
cargo build
cargo run -- --help
```

Note the syntax for *cargo run*.  This executes the server and feeds the command-line arg '--help' to it.


## Configuration

**bookwerx-core-rust** does not do anything by default.  If you want it to do anything useful, you'll need to ensure that it gets the correct configuration options.  You can deliver said options via configuration files, the command line, or the environment.

Execute **bookwerx-core-rust** with the --help option to see the CLI choices.  Each option has a corresponding environment variable.

**bookwerx-core-rust** Uses the following environment variables.  Each of these have a corresponding CLI option:

BCR_CONN - A connection string to connect to the MySQL db.  For example: mysql://root:catfood@192.168.0.103:3306
Notice that there is no trailing \ nor a database name.

BCR_DB - The name of the database to use.

BCR_INIT - A file name for a file that contains SQL that will initialize the db.  If this is present the db will be wiped and reseeded.


### Rocket

**bookwerx-core-rust** uses Rocket as the http server.  [Rocket is configured separately.](https://rocket.rs/v0.4/guide/configuration/#configuration)


### MariaDB

**bookwerx-core-rust** uses MariaDB as for the db.  [This is also configured separately.](https://mariadb.org)

Although **bookwerx-core-rust** is able to drop and rebuild the db from an initial seed, this is a minimal thing.  There are a variety of  settings that people might want to tweak, such as character sets and collation, but the reseeding process does not deal with any of that.  So you may need to examine the configuration of your server to get the particular settings that you want.

