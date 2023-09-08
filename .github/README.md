# ncube - Generalized Hypercube Visualizer

`ncube` allows you to visualize hypercubes of arbitrary dimensions.
It works by rotating the hyperdimensional vertices and applying a chain of perspective projections to them until the 3rd dimension is reached.

It also allows for controlling the simulation in real time, such as tweaking the angular velocity on any plane of rotation.

![Demo](https://raw.githubusercontent.com/ndavd/ncube/main/.github/demo.gif)

## Installation

### Building from source
You need to setup Rust ([cargo](https://doc.rust-lang.org/stable/cargo/)) to build this project from source.
After that, simply clone the project and run the build command:
```
git clone https://github.com/ndavd/ncube
cd ncube
cargo build --release
./target/release/ncube
```
