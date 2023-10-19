#!/usr/bin/env sh

nix shell \
    nixpkgs#rustc \
    nixpkgs#cargo \
    nixpkgs#gcc \
    nixpkgs#jetbrains.idea-community \
    -c idea-community
