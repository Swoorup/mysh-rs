# mysh-rs

## Intro

This is a clone of [swoorup/mysh](https://github.com/Swoorup/mysh) written
in c for learning rust language. This project is also my personal testbed
for rust nightly features :joy:

This shell is severely limited and extremely basic as of now. It supports the following features:

* Pipes `ps aux | grep bash`
* Redirection `ls / > listing`
* Background task `sleep 1&`
* Chaining commands `sleep 5; echo Hello World`

### A small demo

[![asciicast](https://asciinema.org/a/285153.svg)](https://asciinema.org/a/285153)

See [swoorup/mysh/README.md](https://github.com/Swoorup/mysh/blob/master/README.md) for implementation details.

## Building

1. Ensure you have rust nightly toolchain installed (1.54.0-nightly)
2. Run `cargo test` to run all tests
3. Run `cargo run` to start the shell
