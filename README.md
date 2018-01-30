# Rocket/Yew Starter Pack


Rocket is a framework for building web servers in Rust, with a routing API that
feels like Flask (Python). Yew is a frontend framework for writing front end
apps in Rust using cutting-edge (but well-supported) WebAssembly technologies,
and with an API inspired by patterns from Elm and React.

## Getting Started

Install rust with `rustup`. You must use nightly rust.

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

* [maud](lfairy/maud) - html templating 
* [sled](spacejam/sled) - an embedded, ordered key-value store 
* [bincode](TyOverby/bincode) - binary encoding for rust types, so we can put
   stuff in a sled tree

----

**NOTE:** This is a work in progress, but it should build, run and let you get
started. These libraries are moving fast, so if anything is broken feel free to
open an issue or a PR.


