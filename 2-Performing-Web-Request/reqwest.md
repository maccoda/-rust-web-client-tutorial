# Reqwest

## Adding `reqwest` as a Dependency

Adding `reqwest` is very similar to adding `hyper` so I will refer to [that
section] for adding the dependency.

## Making the Request

`reqwest` has a much simpler API for performing get methods as shown in its
[reference], so all we need to work on is adding the headers for the [Github
API].

```rust
extern crate reqwest;
```

[that section]: hyper.md
[reference]: https://docs.rs/reqwest/0.9.2/reqwest/
[Github API]: https://developer.github.com/v3/