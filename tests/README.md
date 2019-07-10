Example:

RUST_BACKTRACE=1 RUST_TEST_THREADS=1 cargo test --test config

This runs just the config.rs test.  It uses a single thread, thus forcing the tests to run sequentially.

Integration tests:

The first part of integration testing is to crank up **bookwerx-core-rust** and/or **dbseed** and study the presence or absence of suitable configuration params as specified via command line args and/or environment variables.

Unfortunately, we can't test these params in isolation.  We want to be able to test for the presence of certain messages relating to the presence or absence of certain configuration parameters.  But as the server starts, it may exit prematurely because of a variety of possible configuration errors that it encounters first.  In order to test a specific configuration param, we must ensure that the rest of the configuration is sufficiently correct to enable the server to proceed to the point at which it will consider the specific configuration under test.  All of this also depends upon the order of operations within the source code.  Changing this will likely change the order of operations.  This is a fragile nuisance, but what better way to deal with it?  Suck it up buttercup and deal with it.

**bookwerx-core-rust** uses the 3rd party crate 'clap' in order to manage the CLI.  We won't bother to test its functionality.  So for example we won't test that --help produces a help screen, we will always use the --long-form of an option and won't test that the -s(short form) also works, nor will we test that --not-a-real-option produces a suitable error message.

When **bookwerx-core-rust** is executed, it must have access to a MySQL server, even during integration testing.  Please examine .travis.yml to see how we easily we use docker to install and run a MariaDB image that we can subsequently use for testing.
  
The testing must therefore have _some_ connection string and _some_ database name.  .travis.yml hardwires some of this and the integration tests will also hardwire suitable values to be compatible.  Given this hardwiring, we must therefore be careful to keep these things in sync.  I don't like this hardwiring, but it's not obvious to me how we can DRY this.  IMHO, tolerating this nuisance is simply the best choice.
