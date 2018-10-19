# Error Handling

Error handling in Rust is done so via the `Result` type as opposed to
exceptions. The result type is a generic enum of the form `Result<T,E>` where
`T` is the type of a successful operation and `E` the type of an erroneous one.
It is an enum as it is split as `Ok(T)` and `Err(E)`.

A few of the functions we are using already present the `Result` type as their
return type. You will find these by most of the places we have added `unwrap` or
`expect`. Now what we want to do is provide some more useful error handling than
just failing the whole program something goes wrong. To do this we will
introduce the `?` operator. The usage of this is covered in the [book] so for
now we will get right into it.

The goal of this section will be to define our own error type that we can
propagate up when any of our inner functions fail. Typically these are defined
in a separate module. Feel free to refer to the [book] for implementation details.

[book]: https://doc.rust-lang.org/book/second-edition/ch09-00-error-handling.html