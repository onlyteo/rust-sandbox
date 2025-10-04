# Rust Sandbox

[![Rust](https://img.shields.io/badge/rust-1.90.0-ffc832.svg?logo=rust&logoColor=ffc832)](https://www.rust-lang.org)
[![Warp](https://img.shields.io/badge/warp-0.4.2-ffc832.svg?logo=warp&logoColor=ffc832)](https://ktor.io)
[![Cargo](https://img.shields.io/badge/cargo-1.90.0-ffc832.svg?logo=rust&logoColor=ffc832)](https://www.rust-lang.org)
[![TypeScript](https://img.shields.io/badge/typescript-5.9.3-3178c6.svg?logo=typescript&logoColor=3178c6)](https://www.typescriptlang.org)
[![React](https://img.shields.io/badge/react-19.2.0-58c4dc.svg?logo=react&logoColor=58c4dc)](https://react.dev)
[![NodeJS](https://img.shields.io/badge/node.js-stable-417e38.svg?logo=nodedotjs&logoColor=417e38)](https://nodejs.org)
[![GitHub license](https://img.shields.io/badge/license-Apache_2.0-e97726.svg)](https://www.apache.org/licenses/LICENSE-2.0)

This repository contains sandbox projects to showcase features of the [Rust framework](https://www.rust-lang.org).

All examples are written in [Rust](https://www.rust-lang.org) and built using [Cargo](https://www.rust-lang.org).

## Examples
Read details about the examples in their respective project roots.

* [Warp REST API](./apps/rust-warp-api-rest)

## Architecture
...

The examples typically consist of a `Frontend` and a `Backend` application.

```mermaid
graph TD
    A[Rust Frontend]:::rust
    B[Rust Backend]:::rust

    A --> B
    
    classDef rust fill: #ffc832, stroke: #000000, color: #000000
```

If the frontend is a JavaScript application then there is often also a `Frontend API` application.

```mermaid
graph TD
    A[React Frontend]:::rust
    B[Rust Frontend API]:::rust
    C[Rust Backend]:::rust
    
    A --> B
    B --> C
    
    classDef react fill: #58c4dc, stroke: #000000, color: #000000
    classDef rust fill: #ffc832, stroke: #000000, color: #000000
```

## Use case
Most examples implement a "hello world" style logic that returns a greeting message when the user inputs a name.

* A user inputs the name "John" and clicks "Submit"
* The system generates a greeting "Hello John!" back to the user
