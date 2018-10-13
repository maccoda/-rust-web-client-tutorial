# Performing the Web Request

The goal of this section is to be able to use our token we now have available to
make a request to the Github API and be able to obtain all the current pull
requests on the Rust repository. We have an option of which crate to use to
achieve this:

1. [`hyper`]: Full HTTP implementation in Rust (client and server)
1. [`reqwest`]: Batteries included crate for performing HTTP requests (built on
   top of `hyper`)

I will let you choose which one you would like to implement, there are a few
differences listed below. As I felt it met the requirements of this project I
have completed it using the higher level library of `reqwest` but have given a
solution for writing it in `hyper` for this section.

* `hyper` is a bit more complicated in terms of number of lines but it does make
  a nice introduction to asynchronous code in Rust
* `reqwest` is very simple in its implementation as we are only doing a single
  get request.
* Using `hyper` will also introduce a few more features of Rust, so if you are
  keen to learn more this is good to try, otherwise if you want to get something
  up and running `reqwest` might be your better option

  [`hyper`]: hyper.md
  [`reqwest`]: reqwest.md

