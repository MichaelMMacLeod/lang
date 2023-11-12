{
  description = "Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        # These are ansi escape sequences for colors in the bash prompt;
        # see https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797
        c0 = ''\[\033[0m\]'';
        c1 = ''${c0}\[\033[4;39m\]'';
        c1b = ''${c0}\[\033[1;39m\]'';
        c2 = ''${c0}\[\033[1;32m\]'';
        makePromptString = name:
          ''${c2}(nix develop .#${name}) ${c1}\u@\h:${c1b}\w${c0}\n\$ '';
        prelude = devShellName: ''
          export PS1='${makePromptString devShellName}'
        '';
      in {
        devShells.miri = pkgs.mkShell {
          shellHook = ''
            ${prelude "miri"}
            echo "\
            This environment contains the tool 'miri' (an interpreter
            for Rust's mid-level-IR), which can help find certain
            cases of undefined behavior in Rust programs. Running
            'miri' requires the nightly version of Rust whereas the
            crates herein are implemented in stable Rust---this is why
            'miri' is provided in a separate devshell.

            To run 'miri' on the tests, use:

              nix develop .#miri -c cargo-miri-test

            Alternatively, once inside this devshell, use:

              cargo miri test
            "
            cargo miri setup
          '';
          buildInputs = with pkgs;
            [
              (rust-bin.selectLatestNightlyWith (toolchain:
                toolchain.minimal.override {
                  extensions = [ "miri-preview" "rust-src" ];
                }))
              (writeShellScriptBin "cargo-miri-test" ''
                cargo miri test
              '')
            ];
        };
        devShells.default = pkgs.mkShell {
          shellHook = ''
            ${prelude "default"}
            echo "\
            Here are some programs available in this environment:

            codium

              Starts the free software version of vscode, instrumented
              with Rust development tools.

            cargo build --release

              Builds a rust crate with optimizations.

            cargo run --release

              Builds and runs a Rust binary crate with optimizations.

            cargo test

              Runs the tests of a crate. The 'miri' tool, which checks
              for certain cases of undefined behavior in Rust programs
              is only available in 'nix develop .#miri'.

            wxmaxima

              Launches a graphical interface to Maxima, a computer
              algebra system. Use this to open documentation files
              with the '.wxmx' extension."
          '';
          buildInputs = with pkgs; [
            rust-bin.stable.latest.default
            (vscode-with-extensions.override {
              vscode = vscodium;
              vscodeExtensions = with vscode-extensions;
                [ rust-lang.rust-analyzer ];
            })
            wxmaxima
          ];
        };
      });
}
