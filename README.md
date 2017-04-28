# Scanner

Scans stdin for secrets and passes safe chunks of bytes back to stdout. You need to have Rust installed.

```
$ echo "This text has no secrets" | cargo run
This text has no secrets
```

```
$ echo "This text abc123def4 has no secrets" | cargo run
This text abc123
```
