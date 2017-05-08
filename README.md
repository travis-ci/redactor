# Redactor

[![Build Status](https://travis-ci.org/travis-ci/redactor.svg?branch=master)](https://travis-ci.org/travis-ci/redactor)

Scans stdin for secrets and passes safe chunks of bytes back to stdout. [You need to have Rust installed](https://www.rustup.rs/).

```
$ export TRAVIS_SECRETS=password
$ cargo run -- "echo 'This text has no secrets'"
This text has no secrets

$ cargo run -- "echo 'This text password has no secrets'"
This text [secure] has no secrets

$ cargo run == "exit 1"
$ echo $?
1
```
