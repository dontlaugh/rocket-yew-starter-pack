# Rocket/Yew Starter Pack

**NOTE:** This is a work in progress.

Rocket is a framework for building web servers in Rust, with a routing API that
feels like Flask (Python). Yew is a frontend framework for writing front end
apps in Rust using cutting-edge (but well-supported) WebAssembly technologies,
and with an API inspired by patterns from Elm and React.

## Getting Started

Install rust with `rustup`. You must use nightly rust.

```
rustup default nightly
```

Build on your current platform

```
./build.sh
```

Or, run locally

```
./run-local.sh
```

Or, build the Rocket server backend as a static binary, using Eric Kidd's 
Docker container

```
./static-binary.sh  # calls server/.container-script.sh
```







