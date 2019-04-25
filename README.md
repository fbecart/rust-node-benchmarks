# Rust Node benchmarks

## Prerequisites

* Node
* [Rustup](https://rustup.rs/)
* [Neon CLI](https://neon-bindings.com/docs/getting-started#install-the-neon-cli)
* [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

## Usage

```sh
npm install
npm run build-subjects
npm run bench
```

## Functions under benchmark

### `sum`

Sums 1 and 2.

The goal of this benchmark is to measure the cost of calling Rust code from JS.

### `sha1`

Computes the hash of a short string.

The goal of this benchmark is to compare a JS implementation to a Rust implementation of the same algorithm.

The js implementation is in pure JS. It probably hasn't been subject to as much optimization as the Rust version.
It definitely isn't as fast as the what the `crypto` node module would provide, but `crypto` uses itself native binaries.

### `fibonacci`

Naive procedural implementation of the fibonacci suite.
