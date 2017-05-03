# Redactor

Scans stdin for secrets and passes safe chunks of bytes back to stdout. [You need to have Rust installed](https://www.rustup.rs/).

```
$ export TRAVIS_SECRETS=password
$ echo "This text has no secrets" | cargo run
This text has no secrets

$ echo "This text password has no secrets" | cargo run
This text [secure] has no secrets
```

**To do:**
- [x] Detect secrets and do not print
- [x] Decide on strategy for secrets of different lengths
- [x] Print replacement e.g. `[secure]` and carry on scanning
- [ ] Build for all platforms with [trust](https://github.com/japaric/trust)
