let
  pkgs = import ./nix;
in
pkgs.mkShell {
  name = "malt";
  buildInputs = with pkgs; [
    cargo
    rust-analyzer

    pkg-config
    openssl

    jq

    niv
    nixpkgs-fmt
    cargo-edit
  ];

  shellHook = "${(import ./.).preCommitChecks.shellHook}";
}
