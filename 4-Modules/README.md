# Organizing with Modules

Here we won't be adding any new features but rather we will be refactoring our
code a touch. The mechanism we are going to use is [modules].

Modules in Rust can be files or they can be inline, in this case we will use the
former making a `model.rs` file to contain our `PullRequest` struct. Following
the chapter of the book on [modules] should hopefully provide the basic syntax.

Remember to change your references to the `PullRequest` struct if you had any!

Through doing this we will also encounter visibility modifiers. By default all
visibility is private to the module unless explicitly stated via the `pub` keyword.

[modules]: https://doc.rust-lang.org/book/second-edition/ch07-00-modules.html