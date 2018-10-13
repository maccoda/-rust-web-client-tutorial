# Deserializing the Response

As you may have seen there is a lot of data we get back from our request, next
we want to be able to use this. First steps to do this will be to be able to
structure the data. The [Github API documents] provides a sample response which
we can follow.

I chose some arbitrary fields that we can print about these pulls requests,
those being:

* number
* status
* title

However feel free to choose your own fields that you think are important or you
want to use. You can always come back and add or remove fields to our data
structure.

## Serde

The standard serialization/deserialization crate is [`serde`]. This has become
quite a mature library and extremely simplifies the problem of deserialization
of our response. `serde` provides an auxillary crate that provides [derive]
macros, removing the need for us to implement the `Deserialize` trait for our
data structure.

### Structs

To deserialize into a data structure we must first make a structure (or
`struct`). Given the fields I chose earlier my struct would look like this:

```rust
struct PullRequest {
    number: usize,
    status: String,
    title: String,
}
```

### Hints:

* Use the `json` function from `reqwest` to be called on our response.
* `Vec` is the list type in Rust's standard library.
* `usize` is an unsigned integer where the size is set by the OS.

[Github API documents]: https://developer.github.com/v3/pulls/#list-pull-requests
[`serde`]: https://serde.rs/
[derive]: https://serde.rs/derive.html