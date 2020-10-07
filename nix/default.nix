let
  sources = import ./sources.nix;
  overlays = [
    (import sources.nixpkgs-mozilla)
    (self: _:
      {
        rustChannel = self.rustChannelOf { channel = "stable"; };
        rustFull = self.rustChannel.rust.override {
          extensions = [
            "clippy-preview"
            "rust-analysis"
            "rust-src"
            "rust-std"
            "rustfmt-preview"
          ];
        };
        cargo = self.rustFull;
        rustc = self.rustFull;
      }
    )
    (self: _: { gitignoreSource = (import sources.gitignore { inherit (self) lib; }).gitignoreSource; })
    (_: _: { nixPreCommitHooks = import sources.nix-pre-commit-hooks; })
    (self: _: { cargo2nix = lockfile: self.callPackage sources.cargo2nix { inherit lockfile; }; })
  ];
in
import sources.nixpkgs { inherit overlays; }
