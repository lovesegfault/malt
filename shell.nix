let
  pkgs = import ./nix;
in
  pkgs.mkShell {
    name = "malt";
    buildInputs = with pkgs; [
      rustFull
      niv
      nixpkgs-fmt
      cargo-edit
    ];
  }
