# Hyper

## Adding `hyper` as a Dependency

Adding dependencies is as simple as most modern languages/build tools. Simply go
into the `Cargo.toml` file and add `hyper` under the `dependencies` key (if
TOML is unfamiliar have a look at the [specification]).

We need to know however which version of `hyper` we want though, how do we find
that out? Rust's central repository is called **[crates.io]**. Here you can find
the current versions of all the crates available, as well as access to the API
documentation, it is also the home of the [documentation] of the `Cargo.toml` file
as well. Search for `hyper` and add that version to your `Cargo.toml`.

*At the time of writing the latest version is 0.12.11*

## Making a Hyper Client

First thing required is to add the crate to our program.

```rust
extern crate hyper;
```

*Note: 2018 Edition of Rust no longer requires this statement*

To actually implement the client we will start with the [client documentation]
as well as the [API reference]. Before we actually write anything we will need
to understand the [Github API], the aspects we need to implement:

1. Adding the authorization token
1. Finding the URI structure to list all pull requests on a repository

Once we understand these we can get to implementing our first cut of the client.
The first new introduction will be imports, these are done through the `use`
keyword, typically placed at the top of the module. The following is a small
snippet to get you started:

```rust
extern crate hyper;

use hyper::Client;
```

[`hyper`]: https://hyper.rs/
[specification]: https://github.com/toml-lang/toml
[crates.io]: https://crates.io/
[documentation]: https://doc.rust-lang.org/cargo/reference/manifest.html
[client documentation]: https://hyper.rs/guides/client/basic/
[API reference]: https://docs.rs/hyper/0.12.11/hyper/client/index.html
[Github API]: https://developer.github.com/v3/