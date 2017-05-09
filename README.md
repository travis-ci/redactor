# Redactor

[![Build Status](https://travis-ci.org/travis-ci/redactor.svg?branch=master)](https://travis-ci.org/travis-ci/redactor)

Scans stdin and stderr for secrets and passes safe chunks of bytes back to stdout.

When no secrets are set, the output is passed straight through, with no attempt at scanning.

```
$ cargo run -- "echo 'Expose my password'"
Expose my password
```

When secrets are set, they are scanned and redacted. Secrets should be set in a sub-process to avoid leaking.

```
$ (export TRAVIS_SECRETS=password; cargo run -- "echo 'Expose my password'")
Expose my [secure]
```

The exit code is preserved too.

```
$ (export TRAVIS_SECRETS=password; cargo run -- "ruby -e 'raise \"password\"'")
-e:1:in `<main>': [secure] (RuntimeError)
$ echo $?
1
```

## Development

Install Rust nightly via [rustup](https://www.rustup.rs/).

Run the tests:

```
$ cargo test
```

Run the command (this builds a debug release in ./target/debug and then executes it):

```
$ cargo run -- "echo hello"
```

You can also run it against fixtures, which test streaming input.

```
$ (export TRAVIS_SECRETS=google; cargo run -- "./fixtures/build.sh")
```

```
$ (export TRAVIS_SECRETS=google; cargo run -- "ruby ./fixtures/stream.rb")
```

## Releasing

Pushing a git tag will automatically package and add to the [releases page](https://github.com/travis-ci/redactor/releases). Make sure to update the version in `Cargo.toml` and push both `Cargo.toml` and `Cargo.lock` beforehand.
