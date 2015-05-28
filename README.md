# cargo-deps

Generates images of dependency graphs for crates on crates.io

## Example

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

```
cargo run
```

To deploy to Heroku, make sure to set the buildpack as follows:

```
heroku buildpacks:set https://github.com/ddollar/heroku-buildpack-multi.git
```

## License

Licensed under version 2 of the Apache license.
