# Choosing the stack

Know that we have an overview of what we will build, let us think a moment about what we will need.

## The database 

We will need some kind of way to store our users, movies, group of user ...etc. This is obviously a use case for relational database.

In web application development we don't like to write raw SQL and usually uses ORMs. Rust doesn't have many of them yet (see [here](http://www.arewewebyet.org/topics/database/#orms)) but [diesel](http://diesel.rs/) is very mature already.  

If you come from the Java world, think of it as a drop in replacement for hibernate + liquidbase/flyway, it handles both SQL migration and mapping with your datastructure. 

## The Http stack

Nowadays the idiomatic way to build a HTTP application is to make a restfull api for the backend application and then build a frontend with a javascript framework.
On the backend side the state of the art is to conform to the [OpenApi Standard](http://spec.openapis.org/oas/v3.0.2) wich is really long and boring. [This article](https://blog.octo.com/en/design-a-rest-api/) sumarize it quite well and will try to conform as much as we can to its guidelines.

Okay but that's just a design thing, now we want to pick up the best rust framework to do it. This is where it gets complicated... 

In other languages the most reliable framworks as been identified a long time ago, some of them are backed by big compagnies: Spring, Django, Symphony...etc. 
Well the rust ecosystem is not that mature yet. As I am writing this line, there is about  [20 web framework available](http://www.arewewebyet.org/topics/frameworks/), and there will probably be more tomorow. Although we cannot predict wich will last, two of them are getting their head out of the pack : [Rocket](https://rocket.rs/) which is fairly easy to use, annotation driven and [Actix](https://actix.rs/) based on a actor system model.

Both would fit our needs here, but we are trying to build the petstore of rust, and Rocket as one major issue: it's using the nighly rust release. So Actix it will be.  
