{
  description = "wordle but it's nix functions";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    noogle.url = "github:nix-community/noogle/86bbf6b011b550c0f8eabf914034a22d57458d9c"; # latest commit is broken
  };

  outputs =
    {
      self,
      nixpkgs,
      noogle,
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
    in
    {
      packages = forAllSystems (pkgs: rec {
        default = nixdle;
        nixdle = pkgs.callPackage ./nix/default.nix { };
        data = pkgs.callPackage ./nix/data.nix {
          inherit (noogle.packages.${pkgs.stdenv.hostPlatform.system})
            pasta
            pesto
            salt
            ;
        };
      });

      devShells = forAllSystems (pkgs: {
        default = pkgs.callPackage ./nix/shell.nix { };
      });

      formatter = forAllSystems (pkgs: pkgs.callPackage ./nix/formatter.nix { });
    };
}
