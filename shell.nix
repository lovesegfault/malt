let
  pkgs = import ./nix;
in
  pkgs.mkShell {
    name = "malt";
    buildInputs = with pkgs; [
      rustFull

      pkg-config
      openssl

      jq

      niv
      nixpkgs-fmt
      cargo-edit
    ];
  }
