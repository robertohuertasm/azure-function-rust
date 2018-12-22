# azure-function-rust

Example of how to build an `Azure Function` using Rust.

If you need more information you can read my [Azure Functions written in Rust](https://robertohuertas.com/2018/12/22/azure-function-rust/) blog post where I describe all the steps I took to build and publish an Azure Function in Rust.

## How to build it

This project is using [azure-functions-sdk](https://github.com/peterhuene/azure-functions-rs/) from [Peter Huene](https://github.com/peterhuene), a senior Software Engineer on the .NET Core Team.

You can go straight ahead to the [azure-functions-sdk](https://github.com/peterhuene/azure-functions-rs/) repo in order to get more insights on how this works.

Basically, you have to be using `nightly` and have installed this:

```sh
cargo install azure-functions-sdk
```

In order to build this:

```sh
cargo func build

```

If you just want to run it:

```sh
cargo func run
# then browse to http://localhost:8080/api/hello
```
