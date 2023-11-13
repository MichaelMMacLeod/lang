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
        c0 = "\\[\\033[0m\\]";
        c1 = "${c0}\\[\\033[4;39m\\]";
        c1b = "${c0}\\[\\033[1;39m\\]";
        c2 = "${c0}\\[\\033[1;32m\\]";
        makePromptString = name:
          "${c2}(nix develop .#${name}) ${c1}\\u@\\h:${c1b}\\w${c0}\\n\\$ ";
        prelude = devShellName: ''
          export PS1='${makePromptString devShellName}'
        '';
        stable = pkgs.rust-bin.stable.latest.default;
        nightly = (pkgs.rust-bin.selectLatestNightlyWith (toolchain:
          toolchain.minimal.override {
            extensions = [ "miri-preview" "rust-src" ];
          }));
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
          buildInputs = with pkgs; [
            nightly
            (writeShellScriptBin "cargo-miri-test" ''
              cargo miri test
            '')
          ];
        };
        devShells.default = pkgs.mkShell {
          shellHook = ''
            ${prelude "default"}
            devshell.current.programs
          '';
          buildInputs = with pkgs; [
            stable
            cargo-watch
            (vscode-with-extensions.override {
              vscode = vscodium;
              vscodeExtensions = with vscode-extensions;
                [ rust-lang.rust-analyzer ];
            })
            wxmaxima
            (writeShellScriptBin "devshell.current.programs" ''
              echo "\
              devshells.current.programs

                Displays this information.

              devshells.list

                Lists the available devshells.

              cargo

                The Rust package manager (stable release version).

              codium

                The free software version of vscode, instrumented with
                Rust development tools.

              wxmaxima

                A graphical interface to Maxima, a computer algebra
                system. Use this to open documentation files with the
                '.wxmx' extension.

              check-with-optimizations    (cargo check --release)
              build-with-optimizations    (cargo build --release)
              run-with-optimizations      (cargo run --release)
              test-with-optimizations     (cargo test --release)
              document-with-optimizations (cargo doc --release)

                Check, build, run, test, or document the Rust crate in
                the current directory with optimizations turned
                on. Arguments to these commands are passed to 'cargo';
                see 'cargo --help' for the possible arguments.

              check-on-save    (cargo check)
              build-on-save    (cargo build)
              run-on-save      (cargo run)
              test-on-save     (cargo test)
              document-on-save (cargo doc)

                Automatically check, build, run, test, or document the
                Rust crate in the current directory whenever a file in
                the current directory or any subdirectory is
                saved. Arguments to these commands are passed to
                'cargo'; see 'cargo --help' for the possible
                arguments."
            '')
            (writeShellScriptBin "devshells.list" ''
              echo "\
              nix develop
              nix develop .#default

                Suitable for Rust development in the stable release
                channel.

              nix develop .#miri

                Contains 'cargo miri test', a tool which may help
                discover undefined behavior in the tests of the crate
                in the current directory. 'miri' is only available on
                the nightly release channel which is why it is
                provided in a seperate devshell."
            '')
            (writeShellScriptBin "check-with-optimizations" ''
              cargo check --release $@
            '')
            (writeShellScriptBin "build-with-optimizations" ''
              cargo build --release $@
            '')
            (writeShellScriptBin "run-with-optimizations" ''
              cargo run --release $@
            '')
            (writeShellScriptBin "test-with-optimizations" ''
              cargo test --release $@
            '')
            (writeShellScriptBin "document-with-optimizations" ''
              cargo doc --release $@
            '')
            (writeShellScriptBin "check-on-save" ''
              cargo watch --shell 'cargo check $@'
            '')
            (writeShellScriptBin "build-on-save" ''
              cargo watch --shell 'cargo build $@'
            '')
            (writeShellScriptBin "run-on-save" ''
              cargo watch --shell 'cargo run $@'
            '')
            (writeShellScriptBin "test-on-save" ''
              cargo watch --shell 'cargo test $@'
            '')
            (writeShellScriptBin "document-on-save" ''
              cargo doc --open
              cargo watch --shell 'cargo doc $@'
            '')
          ];
        };
      });
}
