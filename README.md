# Introduction

[![Build Status](https://travis-ci.org/bostontrader/bookwerx-core-rust.png?branch=master)](https://travis-ci.org/bostontrader/bookwerx-core-rust)
[![codecov](https://codecov.io/gh/bostontrader/bookwerx-core-rust/branch/master/graph/badge.svg)](https://codecov.io/gh/bostontrader/bookwerx-core-rust)
[![MIT license](http://img.shields.io/badge/license-MIT-brightgreen.svg)](http://opensource.org/licenses/MIT)

The purpose of ***bookwerx-core*** is to provide an API that supports multi-currency
 bookkeeping, using the double-entry bookkeeping model, slightly adapted to squeeze 
 in multiple currencies.  
 
At this time the API is primarily RESTful.  However, this is proving to be too tedious to deal with because of the proliferation of different queries and selection criteria that actual requires.  We have therefore established a minimal GraphQL endpoint as well.  Said endpoint is presently at the hello-world stage but it's humble step in the right direction.
 
 ***bookwerx-core*** is written using [the rust programming language.](https://www.rust-lang.org), It uses [rocket](https://rocket.rs) as its web server with [MySQL](https://www.mysql.com) for... you know what MySQL is for.  Finally, it uses [Juniper](https://github.com/graphql-rust/juniper) as the GraphQL library.

Any application that deals with "money" (fiat, precious metals, cryptocoins) will
quickly encounter the need for bookkeeping.  Rolling your own methods is, as usual,
 easier said than done, so perhaps you can save yourself some grief and enjoy ***bookwerx-core*** instead.

With this API, the user can:

* Perform ordinary CRUD operations on the various bookkeeping objects,
such as accounts, currencies, and transactions.

* Query balance sheet and profit and loss information.

* Perform linting of the bookkeeping objects.


## Getting Started

The easiest way to get started is to explore [***bookwerx-ui***](https://github.com/bostontrader/bookwerx-ui-elm).  It provides an [example UI](http://185.183.96.73:3005/) that demonstrates the API interaction with ***bookwerx-core***.

Using this UI you can connect to a [publicly visible demonstration server](http://185.183.96.73:3003), request an API key for your own use, and generally put the API to work.  The UI also guides you through a proper sequence of API calls.  For example, you cannot define an account until you have defined the currency that said account will use.  The UI will also show you the API requests that it creates as well as the responses that it receives.

You can also connect to the [GraphQL endpoint](http://185.183.96.73:3003/graphql) at the same server.  This will present a GraphiQL view that you can use to explore the server's GraphQL capabilities.  Recall that said endpoint is barely past the hello-world stage, so it is presently of limited usefulness.

You can also send an HTTP POST with a particular GraphQL query:
```
export SERVER=http://localhost:3003/graphql
curl -X POST -H "Content-Type: application/json" -d '{"query": "{ hello }"}' $SERVER
curl -X POST -H "Content-Type: application/json" -d '{"query": "{ currencies }"}' $SERVER
```

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
cargo run --bin server -- --help
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

### MySQL

**bookwerx-core-rust** uses MySQL for the db.  [This is configured separately.](https://dev.mysql.com)

Although **bookwerx-core-rust** is able to drop and rebuild the db from an initial seed, this is a minimal thing.  There are a variety of  settings that you might want to tweak, such as character sets and collation, but the reseeding process does not deal with any of that.  So you may need to examine the configuration of your MySQL server to get the particular settings that you want.

## Rust-Rocket-Juniper-MySQL Integration

There are numerous examples of integrating the Juniper GraphQL library with the Rocket webserver.  Unfortunately said examples use contrived toy databases to do their thing.  Real world use frequently requires the use of a db connection object of some sort.  Injecting said object into the Juniper library proved to be a difficult thing to do.  But we've got that beat now so hopefully our code can serve as a useful example for the next hapless soul trapped in this tarpit.

The basic issue is that... using MySQL as an example...

1. Rocket obtains a connection to the db via it's [own methods](https://rocket.rs/v0.4/guide/state/#databases) which uses a "fairing" and the .attach method, but is documented under "state".  Read the instructions carefully, study the examples, and work your way through it and you can do this.  This is not _just_ a matter of code, it's also an issue of how to inject db connection configuration.

2. Juniper's contortions include the creation of a Database object that it feeds into a GraphQL query which ultimately does the work.  The Database object is a great place to put contrived toy example databases, but it alone is not sufficient to connect to MySQL.

3. Rocket's db connection object and Juniper's Database object can both be injected into the route handler for GraphQL.  That's a promising lead.  So all we have to do is create a field of Juniper's Database struct to store the database connection from Rocket, right?  You're barking up the right tree but you're not there yet.

4. The basic problem is that Juniper's Database object is [immutable for reasons of thread safety](https://github.com/graphql-rust/juniper/issues/5).  One answer is to create the field for the connection in Juniper's Database object, but wrap it thus Arc<Mutex<Option<MyRocketSQLConn>>>  Once you know the secret it seems so easy and obvious.


## Dates and Times

Dealing with dates and times is a bottomless pit of complexity.  I'll make this easier for everybody involved by promulgating the following policy:

A transaction occurs at a single instant in time with said time recorded as any string format suitable to your app.

One practical example would be an ISO-8601 string.  Said strings can have the quantity of seconds recorded to an unknown, but sufficient, quantity of decimal places.  For example: "2019-07-20T15:32:17.00000001Z"  This will get you started and you can run a long time before you outgrow this plan.

## Numbers

Generally, the API sends and receives financial numeric amounts using a decimal floating point system.  Each number is represented as an integer significand and an integer exponent.  In this way we can _exactly_ store and transmit the numbers without being bothered with round-off errors.  It's the job of a UI to perform non-destructive rounding when necessary.