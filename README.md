# Redactor

[![Build Status](https://travis-ci.org/travis-ci/redactor.svg?branch=master)](https://travis-ci.org/travis-ci/redactor)

Scans stdin for secrets and passes safe chunks of bytes back to stdout. [You need to have Rust installed](https://www.rustup.rs/).

When no secrets are set, the output is passed straight through, with no attempt at scanning.

```
$ (export TRAVIS_SECRETS=; cargo run -- "echo 'Expose my password'")
Expose my password
```

```
$ (export TRAVIS_SECRETS=password; cargo run -- "echo 'Expose my password'")
Expose my [secure]
```

Although the script runs in a PTY, the exit code is preserved.

```
$ (export TRAVIS_SECRETS=password; cargo run -- "ruby -e 'raise \"password\"'")
-e:1:in `<main>': [secure] (RuntimeError)
$ echo $?
1
```
