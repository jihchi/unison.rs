# unison.rs

An experimental runtime for unison code, written in rust, compiled to wasm for use in the browser.

## "Counter" interactive web example:

- in ucm, run `pull https://github.com/jaredly/unison-web-example .app_test`
- clone this repo (you'll need to have rust installed), and run `cargo run --release -- pack-all ~/.unison/v1/terms ./example/data/all.bin`
- in the `example` directory, run `yarn` to install dependencies, then `yarn serve`
- open `http://localhost:8080`!

![screenshot](./screenshot.png)