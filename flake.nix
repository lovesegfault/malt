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
    rust = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs = { self, crane, flake-utils, nixpkgs, pre-commit-hooks, rust }:
    let
      systems = [ "aarch64-darwin" "aarch64-linux" "x86_64-darwin" "x86_64-linux" ];
    in
    flake-utils.lib.eachSystem systems (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust.overlays.default ];
        };
        inherit (pkgs) lib;

        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        craneLib = (crane.mkLib pkgs).overrideScope' (_: _: {
          rustc = toolchain;
          cargo = toolchain;
          rustfmt = toolchain;
          clippy = toolchain;
        });

        commonArgs = {
          src = ./.;

          buildInputs = with pkgs; [
            libiconv
          ] ++ lib.optional stdenv.isDarwin darwin.apple_sdk.frameworks.Security;

          nativeBuildInputs = with pkgs; [ pkg-config ];
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
            jq
            nixpkgs-fmt
            rnix-lsp
            rust-analyzer
            statix
            xh
          ];
          shellHook = ''
            ${self.checks.${system}.pre-commit-check.shellHook}
          '';
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
