# Writting a common crate

This might seems pointless to you for know but we are going to write a crate with only Plain Old Rust Structs. 
Since we will need our frontend and backend aplications to exchange HTTP request it will at least save us the duplication of these structs.
In the future if we want to build some tooling around our app, let's say a nice cli tool, or expose our api publicly this will be much easier. 

We will also need to serialize and deserialise to and from json.
For that we will use the amazing [serde](https://serde.rs/) crate. 

First we need to add the serde dependency to our `movie-night-common` crate.
Add the following line to `movie-night-common/Cargo.toml` : 

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
```

That's it you can run `cargo build` and start writing the common crate at the same time. 

For now just append the folowing to `movie-night-common/lib.rs`

{{#include common.rs}}

As you can see we have two Data transfer object here, one for retrieving user with there ids and the other specifically for posting new users and persist there password in database. 
Quite simple actually, for those coming from the Java world, the serde `Serialize` and `Deserialize` annotation can be compared to Jackson.
Actually the annotation syntax is almost the same. 

Now that we have some basic struct to post and retrieve new users, lets go ahead and write a web server! 
