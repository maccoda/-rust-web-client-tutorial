- [Contents of this Workshop](#contents-of-this-workshop)
    - [Getting Started](#getting-started)
    - [Reading the Token](#reading-the-token)
    - [Performing a Web Request](#performing-a-web-request)
    - [[Deserializing the Response]](#deserializing-the-response)
    - [[Organizing the Project with Modules]](#organizing-the-project-with-modules)
    - [[Error Handling]](#error-handling)
    - [[Testing]](#testing)
    - [[Enhancing our Client]](#enhancing-our-client)

# Contents of this Workshop

## [Getting Started](0-Getting-Started/README.md)

*5-10 minutes*

In this section we cover everything you need to get started with Rust. We will
set up the development environment. Ideally complete this prior to the workshop.
Also if you have this set up please skip this section.

## [Reading the Token](1-Reading-Token/README.md)

*20 minutes*

In this section we will start the project by reading the authentication token in
from a file. We will be introduced to working with the standard library.

## [Performing a Web Request](2-Performing-Web-Request/README.md)

*15 minutes*

In this section our goal is to be able to successfully hit the Github API. In
doing so we will learn about dependencies (crates).

## [Deserializing the Response](3-Deserializing/README.md)

*20 minutes*

In this section we want to be able to structure the data returned from the API
call. We will learn how to interact with JSON data (and most other serialization
targets as well). It will touch on macros and their power in deriving implementations.

## [Organizing the Project with Modules](4-Modules/README.md)

*10 minutes*

In this section we will focus on small refactoring using Rust's module system.
We are going to separate our code to split our models and the logic.

## [Error Handling](5-Error-Handling/README.md)

*Added some error handling over unwrap*

## [Testing]

*Find some places to do basic testing*

## [Enhancing our Client]

*Allow it to do more than one function, this will be a good place to restructure
to some more idiomatic code with structs, impls, etc.*