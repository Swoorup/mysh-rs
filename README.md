[![Build Status](https://travis-ci.com/Swoorup/mysh-rs.svg?branch=master)](https://travis-ci.com/Swoorup/mysh-rs)

# mysh-rs
This is a clone of [swoorup/mysh](https://github.com/Swoorup/mysh) written
in c for learning rust language. This project is also my personal testbed
for rust nightly features :joy:

This shell is severely limited and extremely basic as of now. It supports the following features:
* Pipes `ps aux | grep bash`
* Redirection `ls / > listing`
* Background task `sleep 1&`
* Chaining commands `sleep 5; echo Hello World`

A small demo: 
[![asciicast](https://asciinema.org/a/vg4i0zKno7tvMjeQJTD9a1zjT.png)](https://asciinema.org/a/vg4i0zKno7tvMjeQJTD9a1zjT)
