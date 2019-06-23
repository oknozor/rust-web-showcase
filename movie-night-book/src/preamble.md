# Preamble

## Why this book ? 

For those of you who are already familiar with the growing rust ecosystem, you might be aware that there is a ton of awesome resources, tutorial, librairies andframework to create web application with rust. Rust is growing balzingly fast and it can be quite confusing when you start learning the language.

At the moment there are 26,952 crates aivalable on [crates.io](https://www.crates.io/), and probably a new web framework every week. In a way it's awesome to see rust growing bigger every day but it makes it difficult to catch up. 

If some rust web framework are quite mature and perfectly viable for production use, some other are too young. Wasm is certainly the next big thing but it's not usable yet.

The goal of this book is to guides you through building a full web application with the current available tools. More than that we ambition to write a reference application and try to discover some idiomatic way of doing so along the way. In a way we will make the rust [petstore](https://www.oracle.com/technetwork/java/index-136650.html). 

So this is not only a tutorial on how to make a restfull API and a frontend in rust, we will try to enforce some good practice along the way, showcase available tooling..etc. 

## Prerequisites 

Before we start building the application you shall be familiar with rust if you are not already. The best way to get started is most certainly [The Rust Programing Language](https://www.rust-lang.org/book) book. For those of you who prefer concrete example, the rust offictial documentation got you covered with [Rust by  example](https://doc.rust-lang.org/stable/rust-by-example/). 

Besides knowing a bit of rust, you probably want to know more about the rust web ecosystem before starting this book. We will use Actix for the backend and Yew to build the frontend application but you might have other ideas. [Check Are We Web Yet](http://www.arewewebyet.org/) if you want to get a grasp of what's currently aivalable. 


