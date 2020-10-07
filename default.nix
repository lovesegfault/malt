let
  pkgs = import ./nix;
in
{
  malt = pkgs.callPackage
    (
      { stdenv
      , cargo2nix
      , cargo
      , openssl
      , pkgconfig
      }: stdenv.mkDerivation {
        name = "malt";
        src = pkgs.gitignoreSource ./.;

        nativeBuildInputs = [ cargo pkgconfig ];
        buildInputs = [ openssl ];

        preBuild =
          let
            cargoEnv = (cargo2nix ./Cargo.lock).env;
          in
          ''
            mkdir .cargo/
            ln -s ${cargoEnv.cargo-config} .cargo/config
            ln -s ${cargoEnv.vendor} vendor
          '';

        buildPhase = ''
          runHook preBuild
          cargo build --release
        '';

        checkPhase = ''
          cargo test
        '';

        installPhase = ''
          mkdir -p $out
          cp target/release/malt $out/malt
        '';
      }
    )
    { };

  preCommitChecks = pkgs.nixPreCommitHooks.run {
    src = pkgs.gitignoreSource ./.;
    excludes = [ "nix/sources.json" "nix/sources.nix" ];
    hooks = {
      nix-linter.enable = true;
      nixpkgs-fmt.enable = true;
    };
  };
}
