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


### Configuration

**bookwerx-core-rust** does not do anything by default.  If you want it to do anything useful, you'll need to ensure that it gets the correct configuration options.  You can deliver said options via command line options or the environment with the CLI having precedence.

Execute **bookwerx-core-rust** with the --help option to see the CLI choices.  Each option has a corresponding environment variable.

**bookwerx-core-rust** Uses the following environment variables.  Each of these have a corresponding CLI option:

BCR_CONN - A connection string to connect to the MySQL db.

BCR_INIT - The mere presence of this will cause the MySQL db to be initialized. 


### Testing

Integration tests:

The first part of integration testing is to crank up **bookwerx-core-rust** and study the presence or absence of suitable command line args and environment variables.

**bookwerx-core-rust** uses the 3rd party crate 'clap' in order to manage the CLI.  We won't bother to test its functionality.  So for example we won't test that --help produces a help screen, we will always use the --long-form of an option and won't test that the -s(short form) also works, nor will we test that --not-a-real-option produces a suitable error message.  

That said, test...

1. If neither --conn nor BCR_CONN are specified, the server will complain and exit.  We _must have_ a connection string or there's nothing else to do.

2. If BCR_CONN is specified, the startup message will mention it.

3. If both the BCR_CONN and the --conn option are specified, the startup message mentions the value from the connection string.

Now the we know that a connection string must be specified and that providing said string works correctly, provide a suitable string for the following tests:

4. If neither --init nor BCR_INIT are specified, but there is a connection string, the startup message _will not_ say anything about initialization.

5. If --init or BCR_INIT is specified, and there is a connection string, the startup message will mention initialization.
