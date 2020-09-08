let
  sources = import ./sources.nix;
  overlays = [
    (import sources.nixpkgs-mozilla)
    (self: super:
      {
        rustChannel = self.rustChannelOf { channel = "nightly"; };
        rustFull = self.rustChannel.rust.override {
          extensions = [
            "clippy-preview"
            "rust-analysis"
            "rust-analyzer-preview"
            "rust-src"
            "rust-std"
            "rustfmt-preview"
          ];
        };
        cargo = self.rustChannel.rust;
        rustc = self.rustChannel.rust;
      }
    )
    (self: super: { naersk = self.callPackage sources.naersk { }; })
  ];
in
import sources.nixpkgs { inherit overlays; }
