#!/usr/bin/env sh

# Script to patch intellij-rust-native-helper on NixOS.

set -e

NOT_DRY_RUN=$1
RUST_NATIVE_HELPER_BINARY="${HOME}/.local/share/$(fd --base-directory "${HOME}/.local/share" --type x intellij-rust-native-helper | head -n 1)"

if [ "${NOT_DRY_RUN}" = "--not-dry-run" ]; then
    patchelf \
        --set-interpreter "$(nix eval --raw nixpkgs#glibc)/lib64/ld-linux-x86-64.so.2" \
        "${RUST_NATIVE_HELPER_BINARY}"
else
    echo "Would patch:"
    echo
    echo "  ${RUST_NATIVE_HELPER_BINARY}"
    echo
    echo "Use --not-dry-run to do the patch"
fi
