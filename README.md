# Redactor

Scans stdin for secrets and passes safe chunks of bytes back to stdout. [You need to have Rust installed](https://www.rustup.rs/).

```
$ echo "This text has no secrets" | cargo run
This text has no secrets
```

```
$ echo "This text abc123def4 has no secrets" | cargo run
This text abc123
```

**To do:**
- [x] Detect secrets and do not print
- [ ] Decide on strategy for secrets of different lengths
- [ ] Print replacement e.g. `[secure]` and carry on scanning
- [ ] Build for all platforms with [trust](https://github.com/japaric/trust)
