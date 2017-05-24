# Redactor

[![Build Status](https://travis-ci.org/travis-ci/redactor.svg?branch=master)](https://travis-ci.org/travis-ci/redactor)

Scans stdin and stderr for secrets and passes safe chunks of bytes back to stdout.

When no secrets are set, the output is passed straight through, with no attempt at scanning.

```
$ redactor -r "echo 'Expose my password'"
Expose my password
```

When secrets are set, they are scanned and redacted. Secrets should be set in a sub-process to avoid leaking.

```
$ (export TRAVIS_SECRETS=ascii:password; redactor -r "echo 'Expose my password'")
Expose my [secure]
```

Secrets can be set with the option `-s` too. They can be ASCII or Base64-encoded and tagged as such.

```
$ redactor -r "echo 'Expose my password'" -s ascii:Ex -s base64:bXk=
[secure]se [secure]assword
```

The exit code is preserved too.

```
$ (export TRAVIS_SECRETS=ascii:password; redactor -r "ruby -e 'raise \"password\"'")
-e:1:in `<main>': [secure] (RuntimeError)
$ echo $?
1
```

## Development

Install Rust stable via [rustup](https://www.rustup.rs/).

Run the tests:

```
$ cargo test
```

Run the command (this builds a debug release in ./target/debug and then executes it):

```
$ cargo run -- -r "echo hello"
```

You can also run it against fixtures, which test streaming input.

```
$ (export TRAVIS_SECRETS=google; cargo run -- -r "./fixtures/build.sh")
```

```
$ (export TRAVIS_SECRETS=google; cargo run -- -r "ruby ./fixtures/stream.rb")
```

## Releasing

Pushing a git tag will automatically package and add to the [releases page](https://github.com/travis-ci/redactor/releases). Make sure to update the version in `Cargo.toml` and push both `Cargo.toml` and `Cargo.lock` beforehand.
