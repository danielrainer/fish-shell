#!/bin/sh

workspace_root="$(realpath "$(dirname "$0")/..")"

# Variables used at build time

export RUSTFLAGS=-Zsanitizer=address
export RUSTDOCFLAGS=-Zsanitizer=address

export FISH_CHECK_RUST_TOOLCHAIN=nightly
export FISH_CHECK_CARGO_ARGS='-Zbuild-std --features=tsan'
# Build fails if this is not set.
export FISH_CHECK_TARGET_TRIPLE=x86_64-unknown-linux-gnu


# Variables used at runtime

export ASAN_OPTIONS=check_initialization_order=1:detect_stack_use_after_return=1:detect_leaks=1
export LSAN_OPTIONS=verbosity=0:log_threads=0:use_tls=1:print_suppressions=0:suppressions="$workspace_root"/build_tools/lsan_suppressions.txt

export FISH_CHECK_LINT=false
export FISH_CI_SAN=1
export FISH_TEST_MAX_CONCURRENCY=4

"$workspace_root"/build_tools/check.sh
