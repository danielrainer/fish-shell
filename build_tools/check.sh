#!/bin/sh

# Script for building and running our checks.
# You can specify the Rust build profile, e.g.
# `--profile=release-with-debug`
# The default profile is `dev`.

set -ex

build_profile=dev

while [ $# -gt 0 ]; do
  case "$1" in
    --profile=?*)
      build_profile=${1#*=}
      ;;
    *)
      printf "Unknown option: %s\n" "$1"
      exit 1
      ;;
  esac
  shift
done

RUSTFLAGS='-D warnings'; export RUSTFLAGS
RUSTDOCFLAGS='-D warnings'; export RUSTDOCFLAGS

repo_root="$(dirname "$0")/.."
if [ "$build_profile" = dev ]; then
  build_dir="$repo_root/target/debug"
else
  build_dir="$repo_root/target/$build_profile"
fi

cargo build --workspace --all-targets --profile="$build_profile"
PATH="$build_dir:$PATH" "$repo_root/build_tools/style.fish" --all --check
cargo clippy --workspace --all-targets --profile="$build_profile"
cargo test --no-default-features --workspace --all-targets --profile="$build_profile"
cargo test --doc --workspace --profile="$build_profile"
cargo doc --workspace --profile="$build_profile"

"$repo_root/tests/test_driver.py" "$build_dir"
