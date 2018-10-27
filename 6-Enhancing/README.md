# Enhancing our Client

*There will be a fair bit of changes in this section, unfortunately not being so
atomic will make it difficult to see the steps in getting to the solution,
hopefully the steps below assist otherwise please let me know how this can be
improved via issue or pull request.*

## Separating Entry Point

First off we want to separate our entry point code from the code that is
performing our business logic, typical modularization but we want to see how to
do this in Rust. Typically your entry point code will be located in your
`main.rs` or another binary file. The business code is then separated by the
an initial module layer in `lib.rs`. This may seem odd at first but it is very
useful as Rust distinguishes between the creation of a library or a binary,
further it allows you to have multiple binaries (entry points) into the same
library backing, all in the one crate. We won't explore the latter in this
example but we will see how to access your *library layer* of code in this
section.

The best way to start with this is to put the `run` function into your `lib.rs`
file.

## Making a `Client`

We want to make a bit more separation so we will create a `Client` which is
created with a token that will be given a request to execute, allowing us to
reuse the same client for multiple requests if we were to add those in the
future.

To do this we will want to make a `struct` that is initialized with the API token.
This will involve some shuffling from the last step as we pushed everything down
into our `lib.rs`.

## Removing deprecated code

You may have been seeing this warning each time you compile your code:

```
warning: use of deprecated item 'std::env::home_dir': This function's behavior is unexpected and probably not what you want. Consider using the home_dir function from https://crates.io/crates/dirs instead.
  --> src/lib.rs:34:16
   |
34 |     let path = env::home_dir().map(|x| x.join(".tokens/github")).unwrap();
   |                ^^^^^^^^^^^^^
   |
   = note: #[warn(deprecated)] on by default
```

Now whilst we are improving everything it would be good to remove this. I wanted
to leave this in so you could see how useful the compiler messages are and what
the warnings appear as. So following exactly what the message requests we will
use the `dirs` crate to obtain the home directory.