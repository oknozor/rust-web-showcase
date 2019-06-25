# Preamble

This along this book we will write a web application in rust, you can find the final result in [this repository](https://github.com/oknozor/rust-web-showcase).
For the sake of conciseness we will sometimes skip some boiler plate code, if you get lost, don't hesitate to refer to the repository.

## Why this book ? 

For those of you who are already familiar with the growing rust ecosystem, you might be aware that there is a ton of awesome resources, tutorial, librairies andframework to create web application with rust. Rust is growing balzingly fast and it can be quite confusing when you start learning the language.

At the moment there are 26,952 crates (a crate is a rust library) aivalable on [crates.io](https://www.crates.io/), and probably a new web framework every week. In a way it's awesome to see rust growing bigger every day but it makes it difficult to catch up. 

If some rust web framework are quite mature and perfectly viable for production use, some other are too young. 
On the frontend side Wasm is almost certainly the next big thing but it's not stable yet.

The goal of this book is to guides you through building a full web application with the current available tools. It is not another hello world, we will try to write a real web application, from the database layer to the frontend.
Think of it as the Java [petstore](https://www.oracle.com/technetwork/java/index-136650.html) of Rust. 

We will try to enforce some good practice along the way, showcase available tooling..etc. 

## Prerequisites 

Before we start building the application you shall be familiar with rust if you are not already. The best way to get started is most certainly [The Rust Programing Language](https://www.rust-lang.org/book) book. For those of you who prefer practice over theory, the rust offictial documentation got you covered with [Rust by  example](https://doc.rust-lang.org/stable/rust-by-example/). 

Besides knowing a bit of rust, you probably want to know more about the rust web ecosystem before starting this book. We will use [Actix](https://actix.rs/) for the backend and [Yew](https://github.com/DenisKolodin/yew) to build the frontend application. If you want to know more about the current state of the art regarding web development in Rust check [Are We Web Yet](http://www.arewewebyet.org/) to get a grasp of what's currently aivalable. 


