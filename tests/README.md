Examples:

RUST_BACKTRACE=1 RUST_TEST_THREADS=1 cargo test

Run all tests.  This command uses a single thread, thus forcing the tests to run sequentially. We need to do this because I've been a bad boy and made tests that are not independent.

RUST_BACKTRACE=1 RUST_TEST_THREADS=1 cargo test --test server_config

This runs just the config.rs test.

Integration tests:

The first part of integration testing is to crank up **bookwerx-core-rust** and/or **dbseed** and study the presence or absence of suitable configuration params as specified via command line args and/or environment variables.

Unfortunately, we can't test these params in isolation.  We want to be able to test for the presence of certain messages relating to the presence or absence of certain configuration parameters.  But as the server starts, it may exit prematurely because of a variety of possible configuration errors that it encounters first.  In order to test a specific configuration param, we must ensure that the rest of the configuration is sufficiently correct to enable the server to proceed to the point at which it will consider the specific configuration under test.  All of this also depends upon the order of operations within the source code.  Changing this will likely change the order of operations.  This is a fragile nuisance, but what better way to deal with it?  Suck it up buttercup and deal with it.

**bookwerx-core-rust** uses the 3rd party crate 'clap' in order to manage the CLI.  We won't bother to test its functionality.  So for example we won't test that --help produces a help screen, we will always use the --long-form of an option and won't test that the -s(short form) also works, nor will we test that --not-a-real-option produces a suitable error message.

When **bookwerx-core-rust** is executed, it must have access to a MySQL server, even during integration testing.  Please examine .travis.yml to see how we easily we use docker to install and run a MariaDB image that we can subsequently use for testing.
  
The testing must therefore have _some_ connection string and _some_ database name.  .travis.yml hardwires some of this and the integration tests will also hardwire suitable values to be compatible.  Given this hardwiring, we must therefore be careful to keep these things in sync.  I don't like this hardwiring, but it's not obvious to me how we can DRY this.  IMHO, tolerating this nuisance is simply the best choice.

KahunaGrande

This is a test the sends many, many requests to the server in order to the operation of each of the routes, as well as referential integrity constraints between the objects that are created in the db.

But what do we _really_ want to test here? There is a combinatorial explosion of possible variations of requests that defy our effort to test them.  Upon reflection, the following categories help with our analysis.

1. Some requests are so malformed that Rocket will not accept them.  Such as:

    A. Missing routes. These will yield 404 not found responses.

    B. Routes that are recognized but have some other problem such as missing or extraneous parameters
        or parameters that cannot be properly parsed. Rocket will reject these requests. These will yield 422 unprocessable entity.

2. Some requests satisfy Rocket, but yield errors with the db. These requests yield 200 responses, but said responses contain some error message from the db.

3. Some request accomplish their goal.  They yield 200 responses.

That said...

1. We don't want to test anything that yields 404 or 422.  Doing so constitutes testing Rocket and is outside
the scope of this project.

2. We don't want to test _most_ errors from the db. For example if a parameter value is too long.  Doing so constitutes testing the db and is outside the scope of this project.

3. There are however a handful of critical db errors that we should test, such as the operation of referential integrity.  Although certainly part of the expected operation of the db, the proper configuration of these constraints is critical to the operation of this app and must be tested.

4. We _usually_ don't want to test POST or PUT by GETing thereafter. Again, doing so constitutes testing Rocket and/or the db.

5. But we want to perform one GET to verify that it actually works.
