#!/bin/sh

workspace_root="$(realpath "$(dirname "$0")/..")"

. "$workspace_root/build_tools/set_asan_vars.sh"

# Variables used at build time

export FISH_CHECK_RUST_TOOLCHAIN=nightly
export FISH_CHECK_CARGO_ARGS='-Zbuild-std'
# Build fails if this is not set.
export FISH_CHECK_TARGET_TRIPLE=x86_64-unknown-linux-gnu


# Variables used at runtime

export FISH_CHECK_LINT=false
export FISH_TEST_MAX_CONCURRENCY=4

"$workspace_root"/build_tools/check.sh
