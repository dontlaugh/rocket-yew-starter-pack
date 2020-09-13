Note: this repository is archived. I sketched it out at the time because nothing like it existed and I wanted to scratch an itch. 
But now Yew has come a really long way, and keeping up with all the changes + maintaining a backend, however minimal, is just not 
something that interests me anymore. It's irresponsible for me to have this be a "most stars" hit when people search for Yew, when 
it will usually be busted and out of date. Better stuff exists. [Start with the official docs](https://yew.rs/docs/en/intro/)

It can still serve as an example for what I'd consider to be a basic repo layout, but you'll have to rewrite most of the 
internals.

The front-end rust web development space is still evolving, but has already benefitted from the (frankly incredible) work of
so many trailblazers.

The next version of this repo needs to be written by someone new, scratching their own itch. Go do radical shit.

Love, Coleman

---

# Rocket/Yew Starter Pack

[Rocket](https://rocket.rs) is a framework for building web servers in Rust, 
with a routing API that feels like Flask (Python). [Yew](https://github.com/DenisKolodin/yew) 
is a front end framework for writing apps in Rust using cutting-edge 
(but well-supported) WebAssembly technologies, and with an API inspired by 
patterns from Elm and React.

This project adapts the [todoMVC example from the yew repo](https://github.com/DenisKolodin/yew/tree/master/examples/todomvc), 
and periodically syncs local state with a backend Rocket server.

We also include some scripts to demonstrate building the ui and shipping it over
to the server to serve as static assets.

## Getting Started

Install rust with `rustup`. Since we build with `--target=wasm32-unknown-unknown`, and rely on 
other [unstable features](https://doc.rust-lang.org/beta/unstable-book/the-unstable-book.html), you must use nightly rust.

```
rustup default nightly
```

Build both `ui` and `server` on your current platform

```
./build.sh
```

Or, build `ui` and copy the outputs over to `server` to run locally. 

```
./run-local.sh
```

Or, build the Rocket server backend as a static binary, using Eric Kidd's 
Docker container. 

```
./static-binary.sh  # calls server/.container-script.sh
```

## Other Libraries

This project also uses:

* [stdweb](https://github.com/koute/stdweb) and [cargo-web](https://github.com/koute/cargo-web) - core library and 
   tooling that make frameworks like yew possible. You can support koute on Patreon.
* [maud](https://github.com/lfairy/maud) - html templating 
* [sled](https://github.com/spacejam/sled) - an embedded, ordered key-value store 
* [bincode](https://github.com/TyOverby/bincode) - binary encoding for rust types, so we can put
   stuff in a sled tree
