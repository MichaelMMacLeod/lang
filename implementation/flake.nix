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
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in {
        devShells.default = pkgs.mkShell {
          shellHook = ''
            echo "GLUE interpreter development environment commands"
            echo
            echo "  codium   - the free software version of vscode"
            echo "             instrumented with Rust development tools"
            echo "  cargo    - build, test, and run Rust code"
            echo "  wxmaxima - a computer algebra system"
            echo
            echo "If you run into trouble, try ..."
            echo
            echo "  nix develop --ignore-environment"
            echo
            echo "... instead, which will ensure that this shell"
            echo "contains *only* what is specified in flake.nix,"
            echo "i.e., all other commands will not be available and"
            echo "will thus not interfere with this environment."
          '';
          buildInputs = with pkgs; [
            rust-bin.stable.latest.default
            (vscode-with-extensions.override {
              vscode = vscodium;
              vscodeExtensions = with vscode-extensions; [
                rust-lang.rust-analyzer
              ];
            })
            wxmaxima
          ];
        };
      }
    );
}
