{ self, ... }: {
  description "devshell";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-23.05";

  outputs = { self, nixpkgs }:
    let
      eachSystem = nixpkgs.lib.genAttrs (import systems);
    in {
      eachSystem (system: {
        devShell = pkgs.mkShell {
          let
            pkgs = nixpkgs.legacyPackages.${system};
          in {
            buildInputs = with pkgs; [
              rustc
              cargo
              gcc
              jetbrains.idea-community
            ];
          };
        };
      });
    };
}
