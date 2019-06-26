Example:

RUST_BACKTRACE=1 RUST_TEST_THREADS=1 cargo test --test config

This runs just the config.rs test.  It uses a single thread, thus forcing the tests to run sequentially.

Integration tests:

The first part of integration testing is to crank up **bookwerx-core-rust** and study the presence or absence of suitable configuration params as specified via command line args and/or environment variables.

Unfortunately, we can't test these params in isolation.  As the server starts, it may exit prematurely because of a variety of possible configuration errors.  In order to test a specific configuration param, we must ensure that the rest of the configuration is sufficiently correct to enable the server to proceed to the point at which it will consider the specific configuration under test.

**bookwerx-core-rust** uses the 3rd party crate 'clap' in order to manage the CLI.  We won't bother to test its functionality.  So for example we won't test that --help produces a help screen, we will always use the --long-form of an option and won't test that the -s(short form) also works, nor will we test that --not-a-real-option produces a suitable error message.

When **bookwerx-core-rust** is executed, it must have access to a MySQL server, even during integration testing.  Please examine .travis.yml to see how we easily we use docker to install and run a MariaDB image that we can subsequently use for testing.
  
The testing must therefore have _some_ connection string and _some_ database name.  .travis.yml hardwires some of this and the integration tests will also hardwire suitable values to be compatible.  Given this hardwiring, we must therefore be careful to keep these things in sync.  I don't like this hardwiring, but it's not obvious to me how we can DRY this.  IMHO, tolerating this nuisance is simply the best choice.

That said...

config test:

Test that we can provide the correct configuration via a mixture of command-line and the environment.  Other configuration is frequently needed in order to enable the server to proceed to the behavior under test.


1.1 If neither --conn nor BCR_CONN are specified, the server will complain and exit.  We _must have_ a connection string or there's nothing else to do.  No other configuration is necessary.

1.2 If either one of --conn or BCR_CONN are specified, the startup message will mention it.  But the server will terminate with an error because other configuration is missing.

1.3 If both --conn and BCR_CONN are specified, the startup message mentions the value from --conn.  But the server will terminate with an error because other configuration is missing.


2.1 If neither --db nor BCR_DB are specified, the server will complain and exit.  We _must have_ a db name or there's nothing else to do.  For this test we must configure a connection string or the server will terminate prematurely.

Starting from here, we have the minimum required configuration to start the server.  However, the available mysql server may or may not have the configured db.  This difference will affect subsequent operation.

When we run the test via CI, we always start with an empty db, so let's take that path.  If we run the tests manually, we need to remember to make sure that the db we're requesting doesn't exisit.

2.2 If either one of --db or BCR_DB are specified, the startup message will mention it.  But the server will terminate with "Fatal error: Unknown database".

2.3 If both --db and BCR_DB are specified, the startup message mentions the value from --db.  But the server will terminate with "Fatal error: Unknown database".


Now that we know that a connection string and a database name can be specified, provide suitable strings for them, so that the server will proceed to initialization.
 
3.1 If neither --init nor BCR_INIT are specified, the startup message _will not_ say anything about initialization.  But the server will terminate with with "Fatal error: Unknown database". because it has not been initialized yet.

3.2 If either one of --init or BCR_INIT are specified...

Test this variation in the seed file errors...
  
3.2.1 If the seed file, as configured via the command line, doesn't exisit, fatal error.

3.2.2 If the seed file, as configured via the environment, can be read but it contains invalid SQL, fatal error.

3.2.3 Given a good seed file that exists, can be read, and is valid SQL, the server will claim successful initialization.  Do this using a good name via the command line and a bad name via the environment to test that the CLi has precedence.

4.1 If neither --bind nor BCR_BIND are specified, the server will complain and exit.  

4.2 If either one of --bind or BCR_BIND are _incorrectly_ specified, the startup message will mention it.  But the server will terminate with an error message.  Here we test that we can feed the configuration, even if nonsense.  In this case we _want_ nonsense because we don't yet want the server to fully start.


At this point I will accept on faith that we can specify both --bind and BCR_BIND and that --bind will have precedence and if it's a bad bind, the server will squeal.

It's now tempting to turn on the server with a 100% correct configuration and observe its operation.  However when we start the HTTP part of the server, that's a blocking operation for the test.  I have yet to figure out how to test this part.
