# About

Zerotoprod project by LukeMathWalker. But using axum instead of actix-web

# Setup

## Faster linker

### For MacOS Arm

Run:

```bash
> brew install llvm && brew install lld

> which lld

> from the output of lld put into linker
```

Put into cargo config file

```toml
# .cargo/config.toml
[target.aarch64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/opt/homebrew/bin/ld64.lld"]
```
