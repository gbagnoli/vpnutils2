Overview
=========

[![tests](https://github.com/gbagnoli/vpnutils2/actions/workflows/main.yml/badge.svg)](https://github.com/gbagnoli/vpnutils2/actions/workflows/main.yml)


Development
===========

there are git hooks one can use to automatically run checks before commit

```
ln -s $(pwd)/hooks/pre-commit.sh .git/hooks/pre-commit
ln -s $(pwd)/hooks/pre-push.sh .git/hooks/pre-push
```

Make sure you have `cargo audit` installed

```
cargo install cargo-audit
```
