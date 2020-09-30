let
  pkgs = import ./nix;
in
  pkgs.naersk.buildPackage {
    src = ./.;

    nativeBuildInputs = with pkgs; [
      pkg-config
    ];

    buildInputs = with pkgs; [
      openssl
    ];

    targets = [ "malt" ];
  }
