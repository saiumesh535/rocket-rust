## Rocket Rust Web framework example (Under Development)

[Rocket](https://rocket.rs/) is great libarray for creating web server using rust language.

## Motivation
Rust is probably one of the most fascinating programming languages, it is as performant as C/C++ yet having modern features.

This repository would give basic exmaples to build your APIs using rocket.

## Before you start
Before starting with this project, I strongly recommend you to learn basics of rus tfrom their official [docs](https://doc.rust-lang.org/book/) they are pretty good.

## Configuration
I really like [VSCode](https://code.visualstudio.com/) and it has very good extensions for Rust. I strongly recommend you to install following extensions

1. [Rust(rls)](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)
2. [Rust](https://marketplace.visualstudio.com/items?itemName=kalitaalexey.vscode-rust)

## Installtion and start server 🚀🚀

This respository has Two project as below

| S.No          | Project       | Description   |
| ------------- | ------------- | ------------- |
| 1             |  server       |  Server with Rust      |
| 2             |  client        | Web app with  Angular  |

## Notice
Since we are using Rocket framework for our server, it's uses latest rust nightly version. for more information please visit [here](https://rocket.rs/v0.4/guide/getting-started/#installing-rust)

# TL;DR

setup your server with following commands
```
cd server
rustup update && cargo update
rustup override set nightly ( override default to nightly to this project only)
```

## setting up client project
```
cd client
npm install
npm start
```

## Booting Project (Development)

| Project       | Command                   | url                       |
| ------------- | -------------             | -------------             |
| server        |  cd server && cargo run              | http://localhost:8000     |
| client         |  cd client && npm start    | http://localhost:4200     |

## Table of Contents
1. Postgres database connection [PR](https://github.com/saiumesh535/rocket-rust/pull/5)


## Some useful resources
1. Official [docs](https://doc.rust-lang.org/book/)
2. [Awesome-Rust](https://github.com/rust-unofficial/awesome-rust)
3. [Learning Rust](https://github.com/gruberb/learning-rust)
