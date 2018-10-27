# Testing

## Writing a Test

Depending on your software background the testing in Rust can be quite unusual.
Typically we will write the tests in the same file in a separate `tests` module
which will only be compiled during a test run, this is achieved by using the
compiler directive of `#[cfg(test)]`. Using this and the references provided
below we should be able to put in place some simple tests (save for the test
actually reaching the API).

## Running the Tests

As you may have expected to run tests we will again be using Cargo, this time
the command we will want to use is `cargo test`.

## Document Tests

Another great feature of Rust with testing is that documentation comments can
include examples and these examples will be tested against the current behaviour
to ensure your documentation is always in sync with your implementation.


### References
[Writing Tests](https://doc.rust-lang.org/book/second-edition/ch11-01-writing-tests.html)