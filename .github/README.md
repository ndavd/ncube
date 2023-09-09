# ncube - A Generalized Hypercube Visualizer

`ncube` allows you to visualize hypercubes of arbitrary dimensions.
It works by rotating the hyperdimensional vertices and applying a chain of perspective projections to them until the 3rd dimension is reached.
**Everything is generated in real time just from the dimension number.**

Aditional features:
- Real time control of the simulation, such as tweaking the angular velocity factor of any plane of rotation
- Exporting and loading custom configurations as files

![Demo](https://raw.githubusercontent.com/ndavd/ncube/main/.github/demo.gif)

## Installation

### Download the pre-built binaries
Pre-built binaries for Windows, Linux, MacOS can be found in the [releases](https://github.com/ndavd/ncube/releases) page.

### Install from crates.io
Install [cargo](https://doc.rust-lang.org/stable/cargo/) and run the command:
```
cargo install ncube
```

### Install from source
You need to setup Rust (install [cargo](https://doc.rust-lang.org/stable/cargo/)) to build this project from source.
After that, simply clone clone the repository and run the install command:
```
git clone https://github.com/ndavd/ncube
cd ncube
cargo install --path .
```

### Uninstall
```
cargo uninstall ncube
```
