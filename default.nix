let
  pkgs = import ./nix;
in
  pkgs.naersk.buildPackage ./.
