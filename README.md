# cargo-deps

Generates images of dependency graphs for crates on crates.io

## Examples

### rust-rss

### tiny_http

[crate-deps.herokuapp.com/tiny_http](https://crate-deps.herokuapp.com/tiny_http)

![](https://crate-deps.herokuapp.com/tiny_http)

### glutin

[crate-deps.herokuapp.com/glutin](https://crate-deps.herokuapp.com/glutin)

![](https://crate-deps.herokuapp.com/glutin)

### geojson

[crate-deps.herokuapp.com/geojson](https://crate-deps.herokuapp.com/geojson)

![](https://crate-deps.herokuapp.com/geojson)

### Hyper

[crate-deps.herokuapp.com/hyper](https://crate-deps.herokuapp.com/hyper)

![](https://crate-deps.herokuapp.com/hyper)

### Piston

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

All images generated using crate-deps.herokupapp.com are licensed under [CC0](https://creativecommons.org/publicdomain/zero/1.0/)
