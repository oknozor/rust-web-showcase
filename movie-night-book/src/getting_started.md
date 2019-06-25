# Getting started

## Seting up the environment

You will need to install some tools along the book(diesel, mdbook...etc) but for know we will keep it simple.

If you did not installed rust and cargo yet, head up to [the official installation instructions](https://www.rust-lang.org/tools/install) or install it via your favorite package manager. 

You will probably want to install linter and formatter for your favorite IDE, all you need is [here](https://www.rust-lang.org/tools)

## (WIP) 
Before we dive into the HTTP stack and start the interesting stuff there are some boiler plate to be written. 
Both our frontend app and backend app will share some of the datastructure, exchanging user, movies ... ect.

On the other hand we really don't want the frontend to access the database directly. 
We will implement a shared model to pass data between our backend and a database library only accessible to the backend.
We will then implement the HTTP rest api with Actix. 
And finally build the frontend application with Yew.

That's a lot of work so we better keep thing organized, we will use the [cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) feature to keep everything in a single repository but still manage every crate separatly: 

## Seting up the environment

```sh
$ mkdir movie-night
$ touch movie-night/Cargo.toml
```
Then we shall edit the Cargo file so it looks like this : 

```toml
[workspace]

members = [
    "movie-night-db",
    "movie-night-backend",
    "movie-night-frontend",
]
```

You can then create all the crates needed : 

```sh
$ cargo new --lib movie-night-db
$ cargo new movie-night-backend
$ cargo new movie-night-frontend
```

Least but not last, lets add a travis build, just crate the following `.travis.yml` at the root of your project : 

```yaml
language: rust
rust:
  - stable

matrix:
  allow_failures:
    - rust: nightly
  fast_finish: true

```


