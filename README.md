# cargo-deps

Generates images of dependency graphs for crates on crates.io

## Example

### Hyper

![](https://crate-deps.herokuapp.com/hyper)

### Piston

![](https://crate-deps.herokuapp.com/piston)

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
