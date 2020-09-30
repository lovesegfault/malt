let
  pkgs = import ./nix;
in
  pkgs.naersk.buildPackage {
    src = ./.;
    targets = [ "malt" ];
  }
