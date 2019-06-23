# Prerequisites

You will need to install some tools along the book(diesel, mdbook...etc) but for know we will keep it simple.

## Intalling the tools

You did not installed rust and cargo yet, head up to [the official installation instructions](https://www.rust-lang.org/tools/install) or install it via your favorite package manager. 

You will probably want to install linter and formatter for your favorite IDE, all you need is [here](https://www.rust-lang.org/tools)


## Hot reload

Finally rust is a compiled language that brings a lot of benefits such as safety and speed, but it comes with the price of long build. We are doing web here so we want stuffs to be reload on the fly. To trigger builds automatically and achieve hot reload we will install `systemfd` and `cargo-watch` :

```sh 
$ cargo install systemfd cargo-watch
```

That's it for now, let's get started!

