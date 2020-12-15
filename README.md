# Rusty Pedestrians

[![Engine CI](https://github.com/cloderic/rusty-pedestrians/workflows/Engine%20CI/badge.svg)](https://github.com/cloderic/rusty-pedestrians/actions)
[![Webapp CI](https://github.com/cloderic/rusty-pedestrians/workflows/Webapp%20CI/badge.svg)](https://github.com/cloderic/rusty-pedestrians/actions)

🚧

Rust pedestrian autonomous navigation engine with a webapp showcase, deployed at
**<https://cloderic.github.io/rusty-pedestrians/>**

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
