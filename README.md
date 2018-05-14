# mysh-rs
This is a clone of [swoorup/mysh](https://github.com/Swoorup/mysh) written in c for learning rust language.

This shell is severely limited and extremely basic as of now. It supports the following features:
* Pipes `ps aux | grep bash`
* Redirection `ls / > listing`
* Background task `sleep 1&`
* Chaining commands `sleep 5; echo Hello World`

A small demo: 
