#!/bin/bash

set -eu
set -o pipefail

pushd "$(git rev-parse --show-toplevel)" >/dev/null
echo "* Running audit"
cargo audit
echo "* Running cargo check" && \
cargo check
echo "* Running cargo fmt"
cargo fmt --all -- --check
echo "* Running cargo clippy"
cargo clippy --all --all-targets -- -Dwarnings -Drust-2018-idioms -Drust-2021-compatibility
echo "* Running cargo test"
cargo test --workspace

echo "* Running shellcheck"
find . -name '*.sh' -print0 | xargs -0 shellcheck

popd >/dev/null
