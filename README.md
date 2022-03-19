# Algorithm Rust

[![run-cargo-test](https://github.com/supdrewin/algorithm-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/supdrewin/algorithm-rs/actions/workflows/rust.yml)
[![pages-build-deployment](https://github.com/supdrewin/algorithm-rs/actions/workflows/pages/pages-build-deployment/badge.svg)](https://github.com/supdrewin/algorithm-rs/actions/workflows/pages/pages-build-deployment)
[![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)

To test it:

``` shell
$ cargo test -r
```

NOTE:

Avoiding to test `sort` package in `DEBUG` mode,
the `quick_sort` will overflow its stack.
