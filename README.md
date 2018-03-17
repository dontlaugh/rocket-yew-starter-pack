# Rocket/Yew Starter Pack

[Rocket](https://rocket.rs) is a framework for building web servers in Rust, 
with a routing API that feels like Flask (Python). [Yew](https://github.com/DenisKolodin/yew) 
is a frontend framework for writing front end apps in Rust using cutting-edge 
(but well-supported) WebAssembly technologies, and with an API inspired by 
patterns from Elm and React.

This project adapts the [todoMVC example from the yew repo](https://github.com/DenisKolodin/yew/tree/master/examples/todomvc), 
and periodically syncs local state with a backend Rocket server.

We also include some scripts to demonstrate building the ui and shipping it over
to the server to serve as static assets.

## Getting Started

Install rust with `rustup`. Since we build with `--target=wasm32-unknown-unknown`, 
you must use nightly rust.

```
rustup default nightly
```

Build both `ui` and `server` on your current platform

```
./build.sh
```

Or, build `ui` and copy the outputs over to `server` to run locally

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

* [stdweb](koute/stdweb) and [cargo-web](koute/cargo-web) - core library and 
   tooling that make frameworks like yew possible. You can support koute on Patreon.
* [maud](lfairy/maud) - html templating 
* [sled](spacejam/sled) - an embedded, ordered key-value store 
* [bincode](TyOverby/bincode) - binary encoding for rust types, so we can put
   stuff in a sled tree

----

**NOTE:** This is a work in progress, but it should build, run and let you get
started. These libraries are moving fast, so if anything is broken feel free to
open an issue or a PR.


