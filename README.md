# Introduction

[![Build Status](https://travis-ci.org/bostontrader/bookwerx-core-rust.png?branch=master)](https://travis-ci.org/bostontrader/bookwerx-core-rust)
[![codecov](https://codecov.io/gh/bostontrader/bookwerx-core-rust/branch/master/graph/badge.svg)](https://codecov.io/gh/bostontrader/bookwerx-core-rust)
[![MIT license](http://img.shields.io/badge/license-MIT-brightgreen.svg)](http://opensource.org/licenses/MIT)

The purpose of ***bookwerx-core*** is to provide an API that supports multi-currency
 bookkeeping, using the double-entry bookkeeping model, slightly adapted to squeeze 
 in multiple currencies.  It uses [rust](https://www.rust-lang.org), [rocket](https://rocket.rs), and [mysql](https://www.mysql.com).

Any application that deals with "money" (fiat, precious metals, cryptocoins) will
quickly encounter the need for bookkeeping.  Rolling your own methods is, as usual,
 easier said than done, so perhaps you can save yourself some grief and enjoy ***bookwerx-core*** instead.

With this API, the user can:

* Perform ordinary CRUD operations on the various bookkeeping objects,
such as accounts, currencies, and transactions.

* Query balance sheet and profit and loss information.

* Perform linting of the bookkeeping objects.


## Getting Started

The best way to get started is to explore ***bookwerx-ui***.  It provides an example UI that demonstrates the API interaction with ***bookwerx-core***.


## Installation

### Prerequisites

* You will need rust (nightly, because that's what Rocket demands).

* You will need git.

* You will need mysql.

The care and feeding of these items are beyond the scope of these instructions...

... but assuming they are correctly installed...

```bash
git clone https://github.com/bostontrader/bookwerx-core-rust.git
cd bookwerx-core-rust
cargo build
cargo run --bin dbseed -- --help
cargo run --bin bookwerx-core-rust -- --help
```
Note the syntax for the *cargo run* commands.  This executes the command and feeds the command-line arg '--help' to it.  Whereupon you can further dissect the operation.

**dbseed** will brain-wipe your db and reseed to a minimal usable condition.  For example:
```bash
cargo run --bin dbseed -- --conn mysql://root:supersecretpassword@172.17.0.2:3306 --dbname somedbname --seed dbseed.sql
```

**server** is the actual server that you're lookin' to use.  The server needs to connect to a db that has been properly seeded, hence the prior step.  As an example for execution of the server:
```bash
cargo run --bin server -- \
    --bind_ip 0.0.0.0 --bind_port 8000 \
    --conn mysql://root:supersecretpassword@172.17.0.2:3306 --dbname somedbname \
    --mode test
```


## Configuration

The binaries of **bookwerx-core-rust** do not do anything by default.  If you want it to do anything useful, you'll need to ensure that they get the correct configuration options.  You can deliver said options via the command line or the environment.

As described above, execute **server** or **dbseed** with the --help option to see the CLI choices.  Each option has a corresponding environment variable.

**dbseed** Uses the following environment variables.  Each of these have a corresponding CLI option:

BCR_CONN - A connection string to connect to the MySQL db.  For example: mysql://root:supersecretpassword@172.17.0.2:3306
Notice that there is no trailing \ nor a database name.

BCR_DBNAME - The name of the database to use.

BCR_SEED - A file name for a file that contains SQL that will initialize the db.  If this is present the db will be brain-wiped and reseeded.

**server** Uses all of the above, except for BCR_SEED.  In addition, it also uses:

BCR_BIND_IP - An IP address for the http server to bind to.

BCR_BIND_PORT - A port for the http server to bind to.

BCR_MODE - Run the server in whatever mode.


### Rocket

**bookwerx-core-rust** uses Rocket as the http server, but it programmatically configures Rocket, so no other Rocket configuration is needed.

### MariaDB

**bookwerx-core-rust** uses MySQL for the db.  [This is configured separately.](https://dev.mysql.com)

Although **bookwerx-core-rust** is able to drop and rebuild the db from an initial seed, this is a minimal thing.  There are a variety of  settings that people might want to tweak, such as character sets and collation, but the reseeding process does not deal with any of that.  So you may need to examine the configuration of your MySQL server to get the particular settings that you want.

## Dates and Times

Dealing with dates and times is a bottomless pit of complexity.  I'll make this easier for everybody involved by promulgating the following policy:

A transaction occurs at a single instant in time with said time recorded as any string format suitable to your app.

One practical example would be an ISO-8601 string.  Said strings can have the quantity of seconds recorded to an unknown, but sufficient, quantity of decimal places.  For example: "2019-07-20T15:32:17.00000001Z"  This will get you started and you can run a long time before you outgrow this plan.

If your app really needs to deal with time-dialation that arises because the different parts of your operation are at different heights in the Earth's gravity well, fine... just do it.  Just code up whatever time makes sense in your personal twilight zone and stuff it into the transaction record.