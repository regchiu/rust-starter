# Automated tests

## Controlling How Tests Are Run

If you don’t want to run the tests in parallel or if you want more fine-grained control over the number of threads used, you can send the `--test-threads` flag and the number of threads you want to use to the test binary. Take a look at the following example:

```shell
$ cargo test -- --test-threads=1
```

If we want to see printed values for passing tests as well, we can tell Rust to also show the output of successful tests with:

```shell
$ cargo test -- --show-output
```

We can pass the name of any test function to cargo test to run only that test.
We can also specify part of a test name, and any test whose name matches that value will be run:

```shell
$ cargo test [TESTNAME]
```

After `#[test]` we add the `#[ignore]` line to the test we want to exclude.

The function is listed as ignored. If we want to run only the ignored tests, we can use:

```shell
$ cargo test -- --ignored
```

If you want to run all tests whether they’re ignored or not, you can run:

```shell
$ cargo test -- --include-ignored
```

## Test Organization

### Unit Tests

### Integration Tests

See: https://doc.rust-lang.org/book/ch11-03-test-organization.html

