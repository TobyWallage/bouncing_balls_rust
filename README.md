
### This is a rust/bevy port of a [python version](https://github.com/TobyWallage/Bouncing_Balls) of a super basic physics simulations. This is basically a `Hello World` application.

- Rust code found in [src](/src) folder.

To compile;

- Build using `cargo build --release --target wasm32-unknown-unknown`
- Create binding for webapp with `wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/bouncing_balls_rust.wasm`
- Change the path inside [index.html](index.html) to the output file if needed
- Start a http server to serve the app with for example `python -m http.server`


A precompiled version is found in the [bounching_balls_rust](/bounching_balls_rust) folder

A version is hosted on my [github.io page](https://tobywallage.github.io/bouncing_balls_rust/)