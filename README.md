# cargo-deps

Generates images of dependency graphs for crates on crates.io

## Usage

To use, make sure your crate:

* has at least one dependency
* is uploaded to crates.io

Then, to generate an image, just go to:

`https://crate-deps.herokuapp.com/<your crate name here>`

## Examples

### [tiny_http](https://crates.io/crates/tiny_http)

[crate-deps.herokuapp.com/tiny_http](https://crate-deps.herokuapp.com/tiny_http)

![](https://crate-deps.herokuapp.com/tiny_http)

### [glutin](https://crates.io/crates/glutin)

[crate-deps.herokuapp.com/glutin](https://crate-deps.herokuapp.com/glutin)

![](https://crate-deps.herokuapp.com/glutin)

### [geojson](https://crates.io/crates/geojson)

[crate-deps.herokuapp.com/geojson](https://crate-deps.herokuapp.com/geojson)

![](https://crate-deps.herokuapp.com/geojson)

### [hyper](https://crates.io/crates/yper)

[crate-deps.herokuapp.com/hyper](https://crate-deps.herokuapp.com/hyper)

![](https://crate-deps.herokuapp.com/hyper)

### [piston](https://crates.io/crates/piston)

[crate-deps.herokuapp.com/piston](https://crate-deps.herokuapp.com/piston)

![](https://crate-deps.herokuapp.com/piston)

## Built with

This project was built using these libraries:

* [tiny-http](https://github.com/frewsxcv/tiny-http)
* [rust-crates-index](https://github.com/frewsxcv/rust-crates-index)

## Setup

To run the server locally:

```
cargo run
```

To deploy to Heroku, make sure to set the buildpack as follows:

```
heroku buildpacks:set https://github.com/ddollar/heroku-buildpack-multi.git
```

## License

All files found in this repository are licensed under version 2 of the Apache license.

All images generated using crate-deps.herokuapp.com are licensed under [CC0](https://creativecommons.org/publicdomain/zero/1.0/)
