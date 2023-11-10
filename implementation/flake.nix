{
  description = "development environment for the GLUE interpreter";

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
            GLUE interpreter is implemented in stable Rust---this is
            why 'miri' is provided in a separate devshell.

            To run 'miri' on the tests, use

              nix develop .#miri -c cargo-miri-test

            or, alternatively, use

              nix develop .#miri
              cargo miri test
            "
            mkdir -p ./target/miri
            export MIRI_SYSROOT=$(realpath ./target/miri)
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
            Welcome to the GLUE interpreter development
            environment. If you run into trouble, try 'nix develop
            --ignore-environment' instead, which will ensure that this
            environment is not polluted by outside state.

            Here are some commands available in this environment that
            may be useful:

            codium

              Starts the free software version of vscode, instrumented
              with Rust development tools.

            cargo run --release

              Builds and runs the GLUE interpreter with optimizations
              turned on.

            cargo test

              Runs the GLUE interpreter tests. To run miri on the
              tests use the environment 'nix develop .#miri' instead.

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
