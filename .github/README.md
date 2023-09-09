# ncube - Generalized Hypercube Visualizer

`ncube` allows you to visualize hypercubes of arbitrary dimensions.
It works by rotating the hyperdimensional vertices and applying a chain of perspective projections to them until the 3rd dimension is reached.
**Everything is generated in real time just from the dimension number.**

It also allows for controlling the simulation, such as tweaking the angular velocity on any plane of rotation.

![Demo](https://raw.githubusercontent.com/ndavd/ncube/main/.github/demo.gif)

## Installation

### Download the pre-built binaries
Pre-built binaries for Windows, Linux, MacOS can be found in the [releases](https://github.com/ndavd/ncube/releases) page.

### Install from source
You need to setup Rust ([cargo](https://doc.rust-lang.org/stable/cargo/)) to build this project from source.
After that, simply clone clone the repository and run the install command:
```
git clone https://github.com/ndavd/ncube
cd ncube
cargo install --path .
```
To uninstall, simply run:
```
cargo uninstall ncube
```
