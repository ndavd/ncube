# ncube - A Generalized Hypercube Visualizer

`ncube` allows you to visualize hypercubes of arbitrary dimensions.
It works by rotating the hyperdimensional vertices and applying a chain of perspective projections to them until the 3rd dimension is reached.
**Everything is generated in real time just from the dimension number.**

<div>
  <img align="center" width="200px" src='https://raw.githubusercontent.com/ndavd/ncube/main/.github/Penteract-q4q5.gif' />
  <img align="center" width="200px" src='https://raw.githubusercontent.com/ndavd/ncube/main/.github/Penteract-q1q4-q3q5.gif' />
  <img align="center" width="200px" src='https://raw.githubusercontent.com/ndavd/ncube/main/.github/Hexeract-q1q4-q2q5-q3q6.gif' />
</div>

### Features
- Real time control of the simulation, such as tweaking the angular velocity factor of any plane of rotation
- Exporting and loading custom configurations as files

![Demo](https://raw.githubusercontent.com/ndavd/ncube/main/.github/demo.gif)

## But what am I actually visualizing?

Let's use the 7-cube demo GIF above as an example. In that specific case, you are looking at:
- _A **3 dimensional perspective projection** of..._
  - _a **4 dimensional perspective projection** of..._
    - _a **5 dimensional perspective projection** of..._
      - _a **6 dimensional perspective projection** of..._
        - _a **7 dimensional hypercube undergoing a rotation** about the **q1q4** (X-W1) and **q2q3** (Y-Z) orthogonal planes._

## Installation

### Download the pre-built binaries
Pre-built binaries for Windows, Linux, MacOS can be found in the [releases](https://github.com/ndavd/ncube/releases) page.

### Install from crates.io
Install [cargo](https://doc.rust-lang.org/stable/cargo/) and run the install command:
```
cargo install ncube
```

### Install from source
Install [cargo](https://doc.rust-lang.org/stable/cargo/), clone the repository and run the install command:
```
git clone https://github.com/ndavd/ncube
cd ncube
cargo install --path .
```

### Uninstall
```
cargo uninstall ncube
```
