# Rusty Pedestrians

[![engine](https://github.com/cloderic/rusty-pedestrians/workflows/engine/badge.svg?branch=main)](https://github.com/cloderic/rusty-pedestrians/actions)

🚧

Rust pedestrian autonomous navigation engine with a webapp showcase.

## Engine

### Build

```console
cd ./engine
wasm-pack build
```

### Tests

#### Unit tests

```console
cd ./engine
cargo test
```

#### Browser side tests

```console
cd ./engine
wasm-pack test --chrome --headless
```

## Webapp

### Install dependencies

```console
cd ./webapp
npm install
```

### Start

```console
cd ./webapp
npm start
```
