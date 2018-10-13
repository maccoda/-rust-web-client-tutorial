# Reading the Token

Our first task will be authentication with Github and being able to add that
authentication into our program. First things first, let's get the token.

## Getting OAuth Token from Github

To be able to authenticate ourselves to Github we will need an OAuth Token.
Navigate to the [personal access token] page and create a new token with scope
of **repo**. Give it a useful description so you remember what it is for.

Once this has been created save this token to a newly created file at
`~/.tokens/github`.

## Reading the token

Now we can get started. We will use the `std::fs::File` API to read this token
into a string and print it.

Rust has quite a well documented [standard library] which we will make use of to
complete this section. Some of the modules we are going to need for this section
are [fs], [env], and [io]. Have a read of these in regards to:
- opening and reading a file
- accessing the home directory

### Building and running our solution

If you are using an IDE that is applying the compilation you may not need this
but you can check how you are going with the borrow checker by running `cargo
check` in the directory containing the `Cargo.toml` file.

[personal access token]:https://github.com/settings/tokens
[standard library]: https://doc.rust-lang.org/std/index.html
[fs]: https://doc.rust-lang.org/std/fs/index.html
[env]: https://doc.rust-lang.org/std/env/index.html
[io]: https://doc.rust-lang.org/std/io/index.html