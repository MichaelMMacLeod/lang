#!/usr/bin/env sh

# Script to patch intellij-rust-native-helper on NixOS.

set -e

nix shell nixpkgs#fd nixpkgs#patchelf --command sh -c "./patch-intellij-rust-native-helper.impl.sh ${1}"
