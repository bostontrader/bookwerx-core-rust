# Introduction

[![Build Status](https://travis-ci.org/bostontrader/bookwerx-core-rust.png?branch=master)](https://travis-ci.org/bostontrader/bookwerx-core-rust)
[![codecov](https://codecov.io/gh/bostontrader/bookwerx-core-rust/branch/master/graph/badge.svg)](https://codecov.io/gh/bostontrader/bookwerx-core-rust)
[![MIT license](http://img.shields.io/badge/license-MIT-brightgreen.svg)](http://opensource.org/licenses/MIT)

The purpose of ***bookwerx-core-rust*** is to provide a primarily RESTful API that supports multi-currency
 bookkeeping, using the double-entry bookkeeping model, slightly adapted to squeeze 
 in multiple currencies.  
 
 ***bookwerx-core*** is written using [the rust programming language.](https://www.rust-lang.org), It uses [rocket](https://rocket.rs) as its web server with [MySQL](https://www.mysql.com) for... you know what MySQL is for.

Any application that deals with "money" (fiat, precious metals, cryptocoins) will
quickly encounter the need for bookkeeping.  Rolling your own methods is, as usual,
 easier said than done, so perhaps you can save yourself some grief and enjoy ***bookwerx-core*** instead.

With this API, the user can:

* Perform ordinary CRUD operations on the various bookkeeping objects,
such as accounts, currencies, and transactions.

* Query balance sheet and profit and loss information.

* Perform linting of the bookkeeping objects.

But wait... there's more!  For those who wish to live dangerously bookwerx-core offers an endpoint that will take a string of custom SQL as a query parameter and send back expected results.  But before you panic and start shouting about SQL Injection attacks please take a moment to read the details of this feature.  There are some safety features to reduce the risk and the general framework of providing said safety gives adventure seekers a start in either tightening, loosening, or eliminating the screws as desired.

## Getting Started

The easiest way to get started is to explore [***bookwerx-ui***](https://github.com/bostontrader/bookwerx-ui-elm).  It provides an [example UI](http://185.183.96.73:3005/) that demonstrates the API interaction with ***bookwerx-core***.

Using this UI you can connect to a [publicly visible demonstration server](http://185.183.96.73:3003), request an API key for your own use, and generally put the API to work.  The UI also guides you through a proper sequence of API calls.  For example, you cannot define an account until you have defined the currency that said account will use.  The UI will also show you the API requests that it creates as well as the responses that it receives.

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

## REST vs GraphQL

In this project we have a choice between using a RESTful API or GraphQL.  Although GraphQL is an interesting contender, after the smoke settled, only the RESTful API emerged from thunderdome.

One major problem with RESTful APIs is that there tends to be a proliferation of endpoints and input parameters in order to accommodate real-world usage.  Naming these things and managing them generally is a tedious (but tractable) exercise.  These woes led us to try using GraphQL.  Unfortunately doing so proved to be a disappointment.  

We encountered the following general intractable issues:

* How can we efficiently execute the GraphQL queries?  Doing so requires some connection to the underlying MySQL db.  And doing that requires that we translate GraphQL queries into MySQL.  This is easier said than done.

* The entire GraphQL ecosystem is generally too complicated for our usage.  A lot of it is very sophisticated and impressive but it's generally tainted by low quality documentation.  Especially the very limited products available for Rust.  Going beyond contrived getting-started examples is too difficult and docs.rs style reference is just not useful.  We consider temptation to dissect a product's parser in order to use it to be a red flag.

We have no wish to bash GraphQL or any of the impressive products that deal with it.  We leave this note here as an archaeological relic so that our progeny and successors can (however unlikely) possibly learn from history and avoid the mistakes of their ancestors.  Perhaps in their time, after their heads have been thawed and their bodies regenerated, they will have access to easier-to-use GraphQL -> SQL tooling.

But until and unless that happens, soldiering on and dealing with the admittedly tedious RESTful managerial issues is tractable and still the easiest path forward for this particular project.

## The /sql endpoint

A few examples to get you started:
```http request
http://localhost:3003/sql?query=SELECT accounts.id FROM accounts&apikey=catfood
http://localhost:3003/sql?query=SELECT accounts.id, currencies.id, currencies.title FROM accounts JOIN currencies ON accounts.currency_id %3d currencies.id&apikey=catfood
http://localhost:3003/sql?query=SELECT accounts_categories.account_id FROM accounts_categories JOIN accounts ON accounts.id%3daccounts_categories.account_id JOIN currencies ON currencies.id%3daccounts_categories.currency_id WHERE accounts_categories.category_id IN (1155, 1165) AND currencies.symbol%3d%22BTC%22 GROUP BY accounts_categories.account_id HAVING COUNT(DISTINCT accounts_categories.category_id)%3d2&apikey=catfood
```
Please notice that we must %encode the equal sign in the query string as %3d.

Please also be aware that in more complex queries you may have parsing problems related to spaces around parentheses.

Because of the woes associated with both ordinary REST and GraphQL we have implemented the /sql endpoint whereby the user can submit a string of SQL and get back expected results.  This feature is not for the faint-of-hearted because it requires knowledge of SQL generally as well as the underlying MySQL schema.  Not to mention the risk of SQL injection attacks.  Nevertheless, there are several safety features in place that can help you control this risk.

First of all, we parse any SQL using [nom-sql](https://github.com/ms705/nom-sql), which is a full-blown SQL parser. An excellent choice for all your SQL parsing needs in general.

Next, we examine the abstract syntax tree (AST) that the parser returns in order to:
 
* Ensure that the query is only a SELECT statement.

* Ensure that all of the fields requested are fully qualified using the table.field syntax and that said tables and fields are present in a white-list.

Finally, the /sql route handler accepts an apikey as a separate parameter and we use that to modify the AST to do our own SQL injection of "WHERE apikey = ? AND (any original where clause)"

The power of this idea comes not from our primitive initial implementation.  Instead, we provide a general framework for parsing, and examining and modifying the resulting AST.  Using this starting point, it would be rather easy to extend.

## Dates and Times

Dealing with dates and times is a bottomless pit of complexity.  We will make this easier for everybody involved by promulgating the following policy:

A transaction occurs at a single instant in time with said time recorded as any string format suitable to your app.

One practical example would be an ISO-8601 string.  Said strings can have the quantity of seconds recorded to an unknown, but sufficient, quantity of decimal places.  For example: "2019-07-20T15:32:17.00000001Z"  This will get you started and you can run a long time before you outgrow this plan.

## Numbers

Generally, the API sends and receives financial numeric amounts using a decimal floating point system.  Each number is represented as an integer significand and an integer exponent.  In this way we can _exactly_ store and transmit the numbers without being bothered with round-off errors.  It's the job of a UI to perform non-destructive rounding when necessary.

## Categories

**bookwerx-core-rust** provides a system of categories so that you may define any number of categories and then tag accounts and transactions with any number of these categories.  It's easy to imagine that we might want to categorize accounts as "assets", "liabilities", etc., and perhaps categorize transactions as "deposits", "transfers", or "withdrawals".  Once you get the hang of this the ideas will flow freely.

This feature is implemented using one table that contains all the defined categories, a join table that relates categories to accounts in a many-to-many relationship, as well as another join table to do the same between categories and transactions.  A category can optionally also have a single parent category. In this this way we can define a hierarchy of categories. 

This is a very general purpose tool.  It would be easy for the user to create a blizzard of categories, lose track of their purposes, apply said categories haphazardly, and to generally get confused.  Managing this complexity is the job for another tool.
