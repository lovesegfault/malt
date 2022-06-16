{
  inputs = {
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs = { self, crane, flake-utils, nixpkgs, pre-commit-hooks, ... }:
    let
      systems = [ "aarch64-darwin" "aarch64-linux" "x86_64-darwin" "x86_64-linux" ];
    in
    flake-utils.lib.eachSystem systems (system:
      let
        pkgs = import nixpkgs { inherit system; };
        inherit (pkgs) lib;

        craneLib = crane.lib.${system};

        commonArgs = {
          src = ./.;

          buildInputs = with pkgs; [
            libiconv
          ];

          nativeBuildInputs = with pkgs; [ ];
        };

        cargoArtifacts = craneLib.buildDepsOnly (commonArgs // { pname = "malt-deps"; });

        clippy = craneLib.cargoClippy (commonArgs // {
          inherit cargoArtifacts;
          cargoClippyExtraArgs = "-- --deny warnings";
        });

        malt = craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });

        coverage = craneLib.cargoTarpaulin (commonArgs // { inherit cargoArtifacts; });
      in
      {
        devShells.default = pkgs.mkShell {
          name = "malt";
          inputsFrom = lib.attrValues self.packages.${system};
          nativeBuildInputs = with pkgs; [
            cargo-edit
            nixpkgs-fmt
            rnix-lsp
            rust-analyzer
            statix
          ];
        };

        packages.default = malt;

        checks = {
          inherit malt clippy;
          pre-commit-check = pre-commit-hooks.lib.${system}.run {
            src = lib.cleanSource ./.;
            hooks = {
              nix-linter.enable = true;
              nixpkgs-fmt.enable = true;
              statix.enable = true;
            };
          };
        } // (lib.optionalAttrs (system == "x86_64-linux") { inherit coverage; });
      });
}
